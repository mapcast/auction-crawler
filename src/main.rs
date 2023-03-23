
use std::{io::Read, process::Command, fs::File};
use encoding::{DecoderTrap, label::encoding_from_whatwg_label, EncoderTrap};
use chrono::{Duration, Datelike};
use postgres::{Client, NoTls};
use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};
use urlencoding::encode_binary;
use serde_json::Value;

struct estate {
    num_id: String,
    kor_id: String,
    court: String,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();
     
    for row in client.query("SELECT id, name FROM TEST_TABLE", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
    
        println!("found person: {} {}", id, name);
    }

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

    let body = format!("srnID=PNO102001&termStartDt={}&termEndDt={}&pageSpec=default40&targetRow=1", search_date, search_date);
    
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36".parse().unwrap());
    //headers.insert(header::COOKIE, "WMONID=mf-Qh8yqvqf; daepyoSidoCd=; daepyoSiguCd=; rd1Cd=; rd2Cd=; realVowel=35207_45207; page=default40; mvmPlaceSidoCd=; mvmPlaceSiguCd=; roadPlaceSidoCd=; roadPlaceSiguCd=; vowelSel=35207_45207; toMul=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20190130005926%2C1%2C20230321%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20210130105713%2C1%2C20230321%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20210130004964%2C2%2C20230328%2CB%5E%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8%2C20190130005285%2C1%2C20230328%2CB%5E; realJiwonNm=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8; JSESSIONID=K4mCRAEnfvyI9pLMHAjTKaYAUksGIJwa0FWpJZMEeifVqwryxhBptdgc3WNhfrKa.amV1c19kb21haW4vYWlzMg==".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf")
        .headers(headers.clone())
        .body(body)
        .send()?
        .text()?;

    let document = Html::parse_fragment(&res);

    let tr_selector = make_selector("tr.Ltbl_list_lvl0");
    let td_selector = make_selector("td");
    
    for (tr_idx, tr) in document.select(&tr_selector).enumerate() {
        if tr_idx == 0 {
            for (td_idx, td) in tr.select(&td_selector).enumerate() {
                if td_idx == 0 {
                    let datas = get_attribute(td.inner_html(), "value").unwrap();
                    let splitted: Vec<&str> = datas.split(",").collect();
                    println!("{} | {}", splitted[0], splitted[1]);
                }
                if td_idx == 1 {
                    let td_inner = td.inner_html();
                    for cap in Regex::new(r"\d{4}타경\d{4}").unwrap().find_iter(&td.inner_html()) {
                        println!("{}", &td_inner[cap.start()..cap.end()]);
                    }
                }
            }
        }
    }
    
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

    Ok(())
}