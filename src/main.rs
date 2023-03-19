use std::{io::Read, process::Command};
use encoding::{DecoderTrap, label::encoding_from_whatwg_label};

fn hex2str(u8vec: &[u8]) -> &str {
    let euckr = encoding_from_whatwg_label("euc-kr").unwrap();
    let decode_string = euckr.decode(u8vec, DecoderTrap::Replace).unwrap();
    return string_to_static_str(decode_string);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


fn main() {
    //let query = r#"curl https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf -d srnID=PNO102000&jiwonNm=%BE%C8%BB%EA%C1%F6%BF%F8&bubwLocGubun=1&jibhgwanOffMgakPlcGubun=&mvmPlaceSidoCd=&mvmPlaceSiguCd=&roadPlaceSidoCd=&roadPlaceSiguCd=&daepyoSidoCd=&daepyoSiguCd=&daepyoDongCd=&rd1Cd=&rd2Cd=&rd3Rd4Cd=&roadCode=&notifyLoc=1&notifyRealRoad=1&notifyNewLoc=1&mvRealGbncd=1&jiwonNm1=%BE%C8%BB%EA%C1%F6%BF%F8&jiwonNm2=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8&mDaepyoSidoCd=&mvDaepyoSidoCd=&mDaepyoSiguCd=&mvDaepyoSiguCd=&realVowel=00000_55203&vowelSel=00000_55203&mDaepyoDongCd=&mvmPlaceDongCd=&_NAVI_CMD=&_NAVI_SRNID=&_SRCH_SRNID=PNO102000&_CUR_CMD=RetrieveMainInfo.laf&_CUR_SRNID=PNO102000&_NEXT_CMD=RetrieveRealEstMulDetailList.laf&_NEXT_SRNID=PNO102002&_PRE_SRNID=PNO102001&_LOGOUT_CHK=&_FORM_YN=Y"#;
    let query = r#"curl https://www.courtauction.go.kr/RetrieveRealEstMulDetailList.laf -d "page=default20&srnID=PNO102000&jiwonNm=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8&bubwLocGubun=1&jibhgwanOffMgakPlcGubun=&mvmPlaceSidoCd=&mvmPlaceSiguCd=&roadPlaceSidoCd=&roadPlaceSiguCd=&daepyoSidoCd=&daepyoSiguCd=&daepyoDongCd=&rd1Cd=&rd2Cd=&rd3Rd4Cd=&roadCode=&notifyLoc=1&notifyRealRoad=1&notifyNewLoc=1&mvRealGbncd=1&jiwonNm1=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8&jiwonNm2=%BC%AD%BF%EF%C1%DF%BE%D3%C1%F6%B9%E6%B9%FD%BF%F8&mDaepyoSidoCd=&mvDaepyoSidoCd=&mDaepyoSiguCd=&mvDaepyoSiguCd=&realVowel=00000_55203&vowelSel=00000_55203&mDaepyoDongCd=&mvmPlaceDongCd=&_NAVI_CMD=&_NAVI_SRNID=&_SRCH_SRNID=PNO102000&_CUR_CMD=RetrieveMainInfo.laf&_CUR_SRNID=PNO102000&_NEXT_CMD=&_NEXT_SRNID=PNO102002&_PRE_SRNID=PNO102001&_LOGOUT_CHK=&_FORM_YN=Y&PNIPassMsg=%C1%A4%C3%A5%BF%A1+%C0%C7%C7%D8+%C2%F7%B4%DC%B5%C8+%C7%D8%BF%DCIP+%BB%E7%BF%EB%C0%DA%C0%D4%B4%CF%B4%D9.&pageSpec=default20&pageSpec=default20&targetRow=1&lafjOrderBy=order+by+maeGiil+desc""#;
    let output = Command::new("cmd")
            .args(["/C", query])
            .output()
            .expect("failed to execute process");
    
    let response = hex2str(&output.stdout);
    println!("{}", response);
    println!("{:?}", output);
}

 