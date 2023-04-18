
use encoding::{DecoderTrap, label::encoding_from_whatwg_label, EncoderTrap};
use chrono::{Duration, Datelike};
use reqwest::header;
use scraper::{Html, Selector, ElementRef};


struct Estate {
    num_id: String,
    kor_id: String,
    court: String,
    category: String,
    address: String,
    specs: String,
    estimated_price: String,
    starting_price: String,
    schedule: String,
    phone_number: String,
    court_number: String,
    failed_count: String,

}

fn hex2str(u8vec: &[u8]) -> &str {
    let euckr = encoding_from_whatwg_label("euc-kr").unwrap();
    let decode_string = euckr.decode(u8vec, DecoderTrap::Replace).unwrap();
    return string_to_static_str(decode_string);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn str2euckr(text: &str) -> Vec<u8> {
    let euckr = encoding_from_whatwg_label("euc-kr").unwrap();
    let decode_string = euckr.encode(text, EncoderTrap::Replace).unwrap();
    decode_string
}

fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

//function from loa(https://crates.io/crates/loa)
fn get_attribute(html_str: String, attr: impl ToString) -> Option<String> {
    let attr = attr.to_string();
    let new_self_vec = html_str
        .split("><")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let new_self = new_self_vec.get(0).unwrap_or(&html_str).replace("\"\"", "");
    let attr_vec = new_self
        .split("\"")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    let mut attr_index: usize = 0;
    for i in 0..attr_vec.len() {
        let s = attr_vec.get(i).expect("get error");
        if s.contains(&attr) {
            attr_index += 1;
            break;
        }
        attr_index = attr_index + 1;
    }
    match attr_vec.get(attr_index) {
        Some(e) => Some(e.to_string()),
        None => None,
    }
}

fn parse_estate(tr: ElementRef) -> Estate {
    
    let mut estate = Estate {
        num_id: String::from(""),
        kor_id: String::from(""),
        court: String::from(""),
        category: String::from(""),
        address: String::from(""),
        specs: String::from(""),
        estimated_price: String::from(""),
        starting_price: String::from(""),
        phone_number: String::from(""),
        schedule: String::from(""),
        court_number: String::from(""),
        failed_count: String::from(""),
    };
    
    let td_selector = make_selector("td");
    for (td_idx, td) in tr.select(&td_selector).enumerate() {
        if td_idx == 0 {
            let datas = get_attribute(td.inner_html(), "value").unwrap();
            let splitted: Vec<&str> = datas.split(",").collect();
            estate.court = splitted[0].to_owned();
            estate.num_id = splitted[1].to_owned();
        }
        if td_idx == 1 {
            let td_inner = td.inner_html();
            let splitted: Vec<&str> = td_inner.split("\n").collect();
            estate.kor_id = splitted[2].trim().replace("<br>", "");
        }
        if td_idx == 2 {
            let td_inner = td.inner_html();
            let splitted: Vec<&str> = td_inner.split("\n").collect();
            estate.category = splitted[2].trim().to_owned();
        }
        if td_idx == 3 {
            let td_inner = td.inner_html();
            let splitted: Vec<&str> = td_inner.split("\n").collect();
            estate.address = splitted[6].trim().replace("</a>", "").to_owned();
            estate.specs = splitted[14].trim().to_owned();
        }
        if td_idx == 5 {
            let td_inner = td.inner_html();
            let splitted: Vec<&str> = td_inner.split("\n").collect();
            estate.estimated_price = splitted[2].trim().replace(",", "").to_owned();
            estate.starting_price = splitted[6].trim().replace(",", "").to_owned();
        }
        if td_idx == 6 {
            let td_inner = td.inner_html();
            let splitted: Vec<&str> = td_inner.split("\n").collect();
            //println!("{}", splitted[4].trim());
            let datas_string = get_attribute(splitted[4].trim().to_string(), "onclick").unwrap().replace("showJpDeptInofTitle(", "").replace(");return false;", "");
            let datas_splitted: Vec<&str> = datas_string.split(",").collect();

            let mut phone_number = datas_splitted[0].replace("'", "").trim().to_owned();
            if phone_number.contains("(") {
                let par_index = phone_number.find("(").unwrap();
                phone_number = phone_number[..par_index].to_owned();
            }
            estate.phone_number = phone_number;

            estate.schedule = datas_splitted[1].replace("'", "").trim().to_owned();
            estate.court_number = datas_splitted[2].replace("'", "").trim().to_owned();
            estate.failed_count = splitted[11].trim().to_owned();
        }
    }
    estate
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut postgres_client = postgres::Client::connect("host=localhost user=postgres", postgres::NoTls).unwrap();

    postgres_client.query(r#"CREATE TABLE IF NOT EXISTS estates (
        num_id VARCHAR(50) PRIMARY KEY,
        kor_id VARCHAR(50),
        court VARCHAR(30),
        category VARCHAR(30),
        address VARCHAR(250),
        specs TEXT,
        estimated_price bigint,
        starting_price bigint,
        phone_number VARCHAR(20),
        schedule VARCHAR(20),
        court_number VARCHAR(50),
        failed_count VARCHAR(20)
    );"#, &[])?;

    /*
    for row in postgres_client.query("SELECT id, name FROM TEST_TABLE", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
    
        println!("found person: {} {}", id, name);
    }
    */
    //let query = r#"curl https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf -d srnID=PNO102000&jiwonNm=%BE%C8%BB%EA%C1%F6%BF%F8&bubwLocGubun=1&jibhgwanOffMgakPlcGubun=&mvmPlaceSidoCd=&mvmPlaceSiguCd=&roadPlaceSidoCd=&roadPlaceSiguCd=&daepyoSidoCd=&daepyoSiguCd=&daepyoDongCd=&rd1Cd=&rd2Cd=&rd3Rd4Cd=&roadCode=&notifyLoc=1&notifyRealRoad=1&notifyNewLoc=1&mvRealGbncd=1&jiwonNm1=%BE%C8%BB%EA%C1%F6%BF%F8&jiwonNm2=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8&mDaepyoSidoCd=&mvDaepyoSidoCd=&mDaepyoSiguCd=&mvDaepyoSiguCd=&realVowel=00000_55203&vowelSel=00000_55203&mDaepyoDongCd=&mvmPlaceDongCd=&_NAVI_CMD=&_NAVI_SRNID=&_SRCH_SRNID=PNO102000&_CUR_CMD=RetrieveMainInfo.laf&_CUR_SRNID=PNO102000&_NEXT_CMD=RetrieveRealEstMulDetailList.laf&_NEXT_SRNID=PNO102002&_PRE_SRNID=PNO102001&_LOGOUT_CHK=&_FORM_YN=Y"#;
    
    let loc = chrono::Local::now() + Duration::days(14);
    let year: String = loc.year().to_string();
    let mut month: String = loc.month().to_string();
    let mut day: String = loc.day().to_string();
    if month.len() == 1 {
        month = format!("0{}", month);
    }
    if day.len() == 1 {
        day = format!("0{}", day);
    }

    let search_date = format!("{}.{}.{}", year, month, day);

    let mut target_row: i32 = 1;

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36".parse().unwrap());

    let mut estates: Vec<Estate> = Vec::new();

    loop {
        let body = format!("srnID=PNO102001&termStartDt={}&termEndDt={}&pageSpec=default40&targetRow={}", search_date, search_date, target_row);
        println!("{}", body);
        //headers.insert(header::COOKIE, "WMONID=mf-Qh8yqvqf; daepyoSidoCd=; daepyoSiguCd=; rd1Cd=; rd2Cd=; realVowel=35207_45207; page=default40; mvmPlaceSidoCd=; mvmPlaceSiguCd=; roadPlaceSidoCd=; roadPlaceSiguCd=; vowelSel=35207_45207; toMul=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20190130005926%2C1%2C20230321%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20210130105713%2C1%2C20230321%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20210130004964%2C2%2C20230328%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20190130005285%2C1%2C20230328%2CB%5E; realJiwonNm=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8; JSESSIONID=K4mCRAEnfvyI9pLMHAjTKaYAUksGIJwa0FWpJZMEeifVqwryxhBptdgc3WNhfrKa.amV1c19kb21haW4vYWlzMg==".parse().unwrap());
        
        let res = client.post("https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf")
            .headers(headers.clone())
            .body(body)
            .send()?
            .text()?;

        if res.contains("검색결과가 없습니다.") {
            break;
        }
    
        let document = Html::parse_fragment(&res);
    
        let tr_selector = make_selector("tr.Ltbl_list_lvl0");
        
        for (tr_idx, tr) in document.select(&tr_selector).enumerate() {
            estates.push(parse_estate(tr));
        }

        //target_row = target_row + 1;

        let delay = std::time::Duration::from_secs(3);
        println!("건전한 스크래핑을 위해 텀을 둡니다. 3초 후 재검색합니다.");
        std::thread::sleep(delay);
        break;
    }

    for est in estates {
        println!("{} | {} | {} | {} | {} | {} | {} | {} | {} | {}", 
            est.num_id,
            est.kor_id,
            est.court,
            est.category,
            est.address,
            est.estimated_price,
            est.starting_price,
            est.phone_number,
            est.court_number,
            est.failed_count
        );
        /* 
        postgres_client.execute(
            r#"INSERT INTO estate(num_id, kor_id, court, category, address, original_price, starting_price, phone_number, court_number, failed_count) 
            values('$1', '$2', '$3', '$4', '$5', $6, $7, '$8', '$9', '$10')"#,
            &[&est.num_id, &est.kor_id, &est.court, &est.category, &est.address, &est.estimated_price,
                 &est.starting_price, &est.phone_number, &est.court_number, &est.failed_count],
        )?;*/
    }
    /* 
    let rows_updated = postgres_client.execute(
        r#"INSERT INTO estate(num_id, kor_id, court, category, address, original_price, starting_price, phone_number, court_number, failed_count) 
        values('$1', '$2', '$3', '$4', '$5', $6, $7, '$8', '$9', '$10')"#,
        &[&estate.num_id, &estate.kor_id, &estate.court, &estate.category, &estate.address, &estate.original_price.to_string(),
             &estate.starting_price.to_string(), &estate.phone_number, &estate.court_number, &estate.failed_count],
    ).unwrap();
    */
    /*
    let court_name = "서울중앙지방법원";
    let court_name_euc_kr_bin = str2euckr(court_name);
    let encoded_court_name = encode_binary(&court_name_euc_kr_bin).to_string();
    //let detail_body = format!("saNo=20190130005285&jiwonNm=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8");
    let detail_body = format!("saNo=20190130005285&jiwonNm={}", encoded_court_name);
    let detail_res = client.post("https://www.courtauction.go.kr/RetrieveRealEstCarHvyMachineMulDetailInfo.laf")
        .headers(headers.clone())
        .body(detail_body)
        .send()?
        .text()?;
    //println!("{:?}", detail_res);

    let court_info = {
        let path = "./court_info.json";
        let json_text = std::fs::read_to_string(path).unwrap();

        serde_json::from_str::<Value>(&json_text).unwrap()
    };

    //println!("{}", court_info["서울중앙지방법원"]["srn_id"]);
    //println!("{}", encoded_court_name);
     */
    Ok(())
}
