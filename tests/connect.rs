use reql::r;

#[tokio::test]
async fn connect() {
    r.connect("localhost").await.unwrap();
}
