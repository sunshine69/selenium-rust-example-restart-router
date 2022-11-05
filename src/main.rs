use thirtyfour::prelude::*;
use std::env;
use std::thread;
use subprocess::*;

fn start_chromdriver() {
    println!("Hello, world!");
    let mut p = Popen::create(&["chromedriver"], PopenConfig {
        stdout: Redirection::Pipe, ..Default::default()
    }).expect("can not spawn");

    // Obtain the output from the standard streams.
    let (out, _err) = p.communicate(None).expect("can not communicate");

    // p.detach();

    // if let Some(exit_status) = p.poll() {
    //     // the process has finished
    //     println!("process ended; status {:?}", exit_status);
    // } else {
    //     // it is still running, terminate it
    //     p.terminate().expect("can not terminate");
    // }
    println!("process ended; output: {}", out.as_ref().unwrap());
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {

    let spawn_chromdriver_thread = thread::spawn(move || {
        start_chromdriver();
    });

    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--headless").expect("can not add args --headless");

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("http://192.168.20.1").await?;
    let form_username = driver.find(By::Id("Frm_Username")).await?;
    form_username.clear().await?;
    form_username.send_keys("admin").await?;

    // Find element from element.
    let form_password = driver.find(By::Id("Frm_Password")).await?;
    let password = env::var("PASSWORD").expect("env var PASSWORD need to be set");
    form_password.send_keys( password ).await?;

    let login_button = driver.find(By::Id("LoginId")).await?;
    login_button.click().await?;

    // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("MenuItem")).await?;
    assert_eq!(driver.title().await?, "H268A");

    driver.query(By::Id("mmManagDiag")).exists().await?;

    let mngt_menu_link = driver.find(By::Id("mmManagDiag")).await?;
    mngt_menu_link.wait_until();
    mngt_menu_link.click().await?;

    driver.query(By::Id("mmManagDevice")).exists().await?;
    let mngt_managedevice = driver.find(By::Id("mmManagDevice")).await?;
    mngt_managedevice.click().await?;

    driver.query(By::Id("Btn_restart")).exists().await?;
    let btn_restart = driver.find(By::Id("Btn_restart")).await?;
    btn_restart.click().await?;

    driver.query(By::Id("confirmOK")).exists().await?;
    let btn_confirm = driver.find(By::Id("confirmOK")).await?;
    btn_confirm.click().await?;

    println!("Reaching here fine");

    // Always explicitly close the browser.
    driver.quit().await?;
    let _p = Popen::create(&["killall", "chromedriver"], PopenConfig {
        stdout: Redirection::Pipe, ..Default::default()
    }).expect("can not killall");

    Ok(())
}
