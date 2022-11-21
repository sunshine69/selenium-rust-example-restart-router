use thirtyfour::prelude::*;
use std::env;
use std::thread;
use std::time::Duration;
use subprocess::*;
use clap::Parser;

#[derive(Parser)]
#[command(name = "router-control")]
#[command(author = "Steve Kieu <msh.computing@gmail.com>")]
#[command(version = "0.2")]
#[command(about = "router-control", long_about = None)]
struct Cli {
    #[arg(short,default_value_t = String::from("restart"), help = String::from("Command to run. Support values: restart, firewall_on, firewall_off"), value_parser = ["restart", "firewall_on", "firewall_off"])]
    command: String,
}

fn start_chromdriver() {
    println!("start_chromdriver");
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

async fn login_router(driver: &WebDriver) -> WebDriverResult<()> {
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
    Ok(())
}

async fn restart_router(driver: &WebDriver) -> WebDriverResult<()> {
    find_click_ele_by_id(driver, "mmManagDiag").await?;
    find_click_ele_by_id(driver, "mmManagDevice").await?;
    find_click_ele_by_id(driver, "Btn_restart").await?;
    find_click_ele_by_id(driver, "confirmOK").await?;
    Ok(())
}

async fn turn_on_firewall(driver: &WebDriver) -> WebDriverResult<()> {
    find_security_filter_page(driver).await?;
    find_click_ele_by_id(driver, "Enable1:IPFilter:1").await?;
    Ok(())
}

async fn turn_off_firewall(driver: &WebDriver) -> WebDriverResult<()> {
    find_security_filter_page(driver).await?;
    find_click_ele_by_id(driver, "Enable0:IPFilter:1").await?;
    Ok(())
}

async fn find_security_filter_page(driver: &WebDriver) -> WebDriverResult<()> {
    find_click_ele_by_id(driver, "mmInternet").await?;
    find_click_ele_by_id(driver, "smSecurity").await?;
    find_click_ele_by_id(driver, "ssmSecFilter").await?;
    Ok(())
}

async fn find_click_ele_by_id(driver: &WebDriver, ele_id: &str) -> WebDriverResult<()> {
    // It is pretty buggy - the exists return false even I can see it in the chrome browser.
    // Do not know why these not work
    println!("started ele id {}", ele_id);
    // let mut count = 0;
    // while count < 5 {
    //         println!("count: {}", count);
    //     if driver.query(By::Id(ele_id)).not_exists().await.unwrap() {
    //         println!("not existed yet, sleeping 3 sec now");
    //         thread::sleep(Duration::new(3, 0) );
    //         count = count + 1;
    //     } else {
    //         println!("existed!");
    //         break;
    //     }
    // }
    // println!("reached here");
    // however this works
    driver.query(By::Id(ele_id)).first().await?;
    let mngt_menu_link = driver.find(By::Id(ele_id)).await?;
    mngt_menu_link.click().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let cli = Cli::parse();

    let _spawn_chromdriver_thread = thread::spawn(move || {
        start_chromdriver();
    });
    thread::sleep(Duration::new(3, 0) );

    let mut caps = DesiredCapabilities::chrome();
    // caps.add_chrome_arg("--headless").expect("can not add args --headless");

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    login_router(&driver).await?;

    match cli.command.as_str() {
        "restart" => restart_router(&driver).await?,
        "firewall_on" => turn_on_firewall(&driver).await?,
        "firewall_off" => turn_off_firewall(&driver).await?,
        _ => println!("Unknown command {}", cli.command),
    }

    println!("Reaching here fine");

    // Always explicitly close the browser.
    driver.quit().await?;
    let _p = Popen::create(&["killall", "chromedriver"], PopenConfig {
        stdout: Redirection::Pipe, ..Default::default()
    }).expect("can not killall");

    Ok(())
}
