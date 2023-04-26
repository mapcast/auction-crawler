# auction-crawler

법원경매(https://www.courtauction.go.kr) 사이트에서 경매 물건 목록을 스크래핑해서 DB에 삽입하는 어플리케이션입니다.

법원경매사이트에 부하를 최소한으로 가하기 위하여 하루에 한번, 14일 후의 데이터를 스크래핑합니다.

기동 시 DB 접속 URL을 ENV파일에 넣어주셔야 합니다.
