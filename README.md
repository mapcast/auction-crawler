# auction-crawler

|num_id|kor_id|court|category|address|specs|estimated_price|starting_price|phone_number|schedule|court_number|failed_count|
|------|------|-----|--------|-------|-----|---------------|--------------|------------|--------|------------|------------|
|20180130006939|2018타경6939|서울중앙지방법원|기타|서울특별시 중구 퇴계로 217, 4층10호|[집합건물 철근콘크리트구조 29.88㎡]|0|0|530-2714|2023.05.10 10:00|제4별관 211호 법정|신건|

법원경매(https://www.courtauction.go.kr) 사이트에서 경매 물건 목록을 스크래핑해서 DB에 삽입하는 어플리케이션입니다.

법원경매사이트에 부하를 최소한으로 가하기 위하여 하루에 한번, 14일 후의 데이터를 스크래핑합니다.

기동 시 DB 접속 URL을 ENV파일에 넣어주셔야 합니다.
