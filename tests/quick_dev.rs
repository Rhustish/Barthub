use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() ->Result<()>{

    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hc").await?.print().await?;

    Ok(())
}