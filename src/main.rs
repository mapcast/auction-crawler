
use std::{io::Read, process::Command};
use encoding::{DecoderTrap, label::encoding_from_whatwg_label};
use chrono::{Duration, Datelike};
use postgres::{Client, NoTls};
use reqwest::header;

fn hex2str(u8vec: &[u8]) -> &str {
    let euckr = encoding_from_whatwg_label("euc-kr").unwrap();
    let decode_string = euckr.decode(u8vec, DecoderTrap::Replace).unwrap();
    return string_to_static_str(decode_string);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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
    println!("{}", month.len());
    if month.len() == 1 {
        month = format!("0{}", month);
    }
    if day.len() == 1 {
        day = format!("0{}", day);
    }

    let search_date = format!("{}.{}.{}", year, month, day);
    /* 
    let query = format!("curl https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf -d \"&srnID=PNO102001&termStartDt={}&termEndDt={}", search_date, search_date);
    let output = Command::new("cmd")
            .args(["/C", &query])
            .output()
            .expect("failed to execute process");
    
    let response = hex2str(&output.stdout);
    println!("{}", response);
     */
    //println!("{:?}", output);

    //srnId=법원 ID
    //term=기간
    //pageSpec, targetRow = page, start
    let body = format!("srnID=PNO102001&termStartDt={}&termEndDt={}&pageSpec=default40&targetRow=1", search_date, search_date);
    
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let req = client.post("https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf")
        .headers(headers.clone())
        .body(body)
        .send()?
        .text()?;
    //println!("{:?}", req);

    //루프 돌려서 검색결과 없을때까지 데이터 긁는 처리

    //상세 검색
    let detail_body = format!("saNo=20190130005285&jiwonNm=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8");
    let detail_req = client.post("https://www.courtauction.go.kr/RetrieveRealEstCarHvyMachineMulDetailInfo.laf")
        .headers(headers.clone())
        .body(detail_body)
        .send()?
        .text()?;
    println!("{:?}", detail_req);
    Ok(())
    /*
    let mut data = "srnID=PNO102000".as_bytes();
    

    let mut easy = Easy::new();
    let mut list = List::new();
    list.append("Content-Type: application/x-www-form-urlencoded").unwrap();
    easy.url("https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf").unwrap();
    easy.http_headers(list).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.write_function(|data| {
        println!("{}", hex2str(data));
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
     */
    //let res = req.send()?.text()?;
    //println!("{}", res);
}