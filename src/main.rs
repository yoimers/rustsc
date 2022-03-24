use std::collections::VecDeque;

// use std::fs::OpenOptions;
// use std::collections::HashMap;
// use crate::domain::infra::user::user_json_repository;
// use crate::domain::models::user::user::UserId;
// use crate::domain::service::scrapes::{
//     get_img_link, get_next_post_page, get_page, get_post_page_list,
// };
// use crate::get_img_link::get_img_link;
// use crate::get_next_post_page::get_next_post_page;
// use crate::get_page::get_page;
// use crate::get_post_page_list::get_post_page_list;
mod domain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let base_url = "https://kemono.party".to_string();
    // let mut next_link_que = VecDeque::new();
    // next_link_que.push_back(base_url + "/post");
    // while let Some(next) = next_link_que.pop_front() {
    //     let post_page = get_page(&next).await?;
    //     if let Some(next) = get_next_post_page(&post_page).await? {
    //         next_link_que.push_back(next);
    //     }
    //     let link_list = get_post_page_list(&post_page).await?;
    //     let mut link_que = VecDeque::from(link_list);

    //     while let Some(link) = link_que.pop_front() {}
    // }

    // let post_page = get_page("https://kemono.party/fanbox/user/49906039/post/3278372").await?;
    // let selector = scraper::Selector::parse("div.post__body  a[href]").unwrap();
    // let img_list = get_img_link(&post_page, &selector)?;
    // img_list.for_each(|e| println!("{:?} {:?}", e.value().attr("href"), e.inner_html().trim()));


    Ok(())
}
