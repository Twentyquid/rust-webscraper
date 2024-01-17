use scraper::{self};
use std::env;
use std::{collections::HashMap, sync::Arc};

use reqwest::{self, cookie::Jar};
use tokio;

// Cookie
// ASPSESSIONIDCGBQSTTD=NAGNAKDCIJGDKJIDGELJNBPL; PHPSESSID=lvtek3e5bemjch7hvaoosljv29
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let userid = &args[1];
    let password = &args[2];
    println!("{:#?}", args);
    let cookie_store = Arc::from(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_store)
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
    // headers.insert(
    //     "Cookie",
    //     "ASPSESSIONIDCGBQSTTD=NAGNAKDCIJGDKJIDGELJNBPL; PHPSESSID=lvtek3e5bemjch7hvaoosljv29"
    //         .parse()?,
    // );

    let mut params = std::collections::HashMap::new();
    params.insert("Userid", &userid);
    params.insert("Password", &password);

    let request = client
        .request(
            reqwest::Method::POST,
            "https://www.rajagiritech.ac.in/stud/ktu/Student/varify.asp",
        )
        .headers(headers.to_owned())
        .form(&params);

    let _response = request.send().await?;
    // let body = response.text().await?;

    // println!("{}", body);
    let mut params2: HashMap<String, String> = std::collections::HashMap::new();
    // params2.insert("Class_Code".to_string(), "2021S1AID".to_string());
    // params2.insert("ACode".to_string(), "17".to_string());
    // let request2 = client
    //     .request(
    //         reqwest::Method::POST,
    //         "https://www.rajagiritech.ac.in/stud/ktu/Student/Activity.asp",
    //     )
    //     .headers(headers)
    //     .form(&params2);

    let mut sem_list: Vec<String> = vec![];
    let sem_request = client.request(
        reqwest::Method::GET,
        "https://www.rajagiritech.ac.in/stud/ktu/Student/Activity.asp",
    );
    let sem_response = sem_request.send().await?;
    let body = sem_response.text().await?;
    let document = scraper::Html::parse_document(&body);
    let option_selector = scraper::Selector::parse(r#"select[name="Class_Code"] option"#).unwrap();
    let options = document.select(&option_selector);
    for option in options {
        sem_list.push(option.inner_html())
    }

    // let response2 = request2.send().await?;
    // let body2 = response2.text().await?;
    // println!("{}", body2);
    let mut count = 0;
    for year_sem in sem_list {
        for i in 11..27 {
            println!("######## {} {} #########", year_sem, i);
            params2.insert("Class_Code".to_string(), year_sem.to_string());
            params2.insert("ACode".to_string(), i.to_string());

            let request2 = client
                .request(
                    reqwest::Method::POST,
                    "https://www.rajagiritech.ac.in/stud/ktu/Student/Activity.asp",
                )
                .headers(headers.to_owned())
                .form(&params2);
            let response2 = request2.send().await?;
            let body = response2.text().await?;

            let document = scraper::Html::parse_document(&body);
            let table_selector = scraper::Selector::parse(".table-striped").unwrap();
            let mut striped_table = document.select(&table_selector);
            if let None = striped_table.nth(0) {
                continue;
            } else {
                // println!("striped table exists");
                let td_selector =
                    scraper::Selector::parse(".table-striped tbody tr:not(:first-child) td.ibox")
                        .unwrap();
                let index_finder_selector =
                    scraper::Selector::parse(".table-striped tbody tr:first-child td").unwrap();
                let index_finder = document.select(&index_finder_selector);
                let index_scale = (index_finder.count() - 3) as i32;
                // println!("index scale: {}", index_scale);
                // let at_data = document.select(&td_selector);
                // let mut second_atdata = document.select(&td_selector);
                // let mut j = index_scale;
                let third_atdata = document.select(&td_selector);
                // let index = at_data.count();

                // let point = second_atdata.nth(index_scale).unwrap();
                // let _ = second_atdata.nth(3).unwrap();
                // println!("{}", point.inner_html());
                // println!("index is: {}", index);

                let mut k = 0;

                for item in third_atdata {
                    // println!("k is: {}", k);
                    let item_string = item.inner_html();
                    if k == index_scale {
                        if item_string != "" {
                            count += item_string.parse::<i32>().unwrap();
                        }
                        k = -2;
                    } else {
                        k += 1;
                    }
                }
            }
            // let point_selector =
            //     scraper::Selector::parse(".table-striped > tbody > tr > .ibox").unwrap();
            // let mut points = document.select(&point_selector);
            // let fragment: Html;
            // match points.nth(0) {
            //     Some(_) => {
            //         println!("{}", points.nth(0).unwrap().inner_html());
            //         // let fragment_string = points.nth(0).unwrap().inner_html();
            //         // fragment = scraper::Html::parse_fragment(&fragment_string);
            //     }
            //     None => continue,
            // }
            // println!("{:#?}", fragment);
            // let act_selectors = scraper::Selector::parse("td").unwrap();
            // let tds = fragment.select(&act_selectors);
            // for item in tds {
            //     println!("inner html is: {}", item.inner_html())
            // }
        }
    }
    println!("points: {}", count);

    Ok(())
}

// activity points link https://www.rajagiritech.ac.in/stud/ktu/Student/Activity.asp
// form data Class_Code=2021S1AID&ACode=11
// table table-striped
// values range from 11 to 27
