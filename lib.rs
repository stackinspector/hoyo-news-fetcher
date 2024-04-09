#![allow(unused)]

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_urlencoded::to_string as to_urlencoded;

const WEB_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36";

const API_HOST: &str = "api-takumi-static.mihoyo.com";
const YS_ORIGIN: &str = "https://ys.mihoyo.com";
const YS_REFERER: &str = "https://ys.mihoyo.com/";
// const SR_ORIGIN: &str = "https://sr.mihoyo.com";
// const SR_REFERER: &str = "https://sr.mihoyo.com/";

pub trait RestApi: Serialize {
    // const METHOD: RestApiRequestMethod;
    // current no need for biz
    const PATH: &'static str;
    type Response: DeserializeOwned;
}

#[derive(Deserialize)]
struct ApiResponse<D> {
    #[serde(rename(deserialize = "retcode"))]
    code: i32,
    message: String,
    data: Option<D>, // null if error
}

// above api abstractions

const LANG_ZH_CN: &str = "zh-cn";
const DEFAULT_PAGE_SIZE: u32 = 100;

#[derive(Clone, Debug, Serialize)]
struct GetYsNewses {
    #[serde(rename(serialize = "iAppId"))]
    app_id: u32,
    #[serde(rename(serialize = "iChanId"))]
    channel_id: u32,
    #[serde(rename(serialize = "iPageSize"))]
    page_size: u32,
    #[serde(rename(serialize = "iPage"))]
    page_n: u32,
    #[serde(rename(serialize = "sLangKey"))]
    lang: &'static str,
}

const YS_APP_ID: u32 = 43;
const YS_CHANNEL_ID: u32 = 719;

impl GetYsNewses {
    fn first() -> Self {
        GetYsNewses {
            app_id: YS_APP_ID,
            channel_id: YS_CHANNEL_ID,
            page_size: 1,
            page_n: 1,
            lang: LANG_ZH_CN,
        }
    }

    fn page(page_n: u32) -> Self {
        GetYsNewses {
            app_id: YS_APP_ID,
            channel_id: YS_CHANNEL_ID,
            page_size: DEFAULT_PAGE_SIZE,
            page_n,
            lang: LANG_ZH_CN,
        }
    }
}

impl RestApi for GetYsNewses {
    const PATH: &'static str = "/content_v2_user/app/16471662a82d418a/getContentList";
    type Response = Newses;
}

/*

#[derive(Clone, Debug, Serialize)]
struct GetSrNewses {
    #[serde(rename(serialize = "iPage"))]
    page_n: u32,
    #[serde(rename(serialize = "iPageSize"))]
    page_size: u32,
    #[serde(rename(serialize = "sLangKey"))]
    lang: &'static str,
    #[serde(rename(serialize = "isPreview"))]
    is_preview: i32,
    // #[serde(rename(serialize = "iAppId"))]
    // app_id: u32,
    #[serde(rename(serialize = "iChanId"))]
    channel_id: u32,
}

const SR_IS_PREVIEW: i32 = 0;
const SR_CHANNEL_ID: u32 = 255;

impl GetSrNewses {
    fn first() -> Self {
        GetSrNewses {
            page_n: 1,
            page_size: 1,
            lang: LANG_ZH_CN,
            is_preview: SR_IS_PREVIEW,
            channel_id: SR_CHANNEL_ID,
        }
    }

    fn page(page_n: u32) -> Self {
        GetSrNewses {
            page_n,
            page_size: DEFAULT_PAGE_SIZE,
            lang: LANG_ZH_CN,
            is_preview: SR_IS_PREVIEW,
            channel_id: SR_CHANNEL_ID,
        }
    }
}

impl RestApi for GetSrNewses {
    const PATH: &'static str = "/content_v2_user/app/1963de8dc19e461c/getContentList";
    type Response = Newses;
}

*/

#[derive(Clone, Debug, Deserialize)]
struct Newses {
    #[serde(rename(deserialize = "iTotal"))]
    total_count: u32,
    list: Vec<News>,
}

#[derive(Clone, Debug, Deserialize)]
struct News {
    #[serde(rename(deserialize = "sChanId"))]
    channel_id: Vec<String>, // as u32
    #[serde(rename(deserialize = "sTitle"))]
    title: String,
    #[serde(rename(deserialize = "sIntro"))]
    intro: String,
    #[serde(rename(deserialize = "sUrl"))]
    url: String,
    #[serde(rename(deserialize = "sAuthor"))]
    author: String,
    #[serde(rename(deserialize = "sContent"))]
    content: String,
    #[serde(rename(deserialize = "sExt"))]
    ext: String,
    #[serde(rename(deserialize = "dtStartTime"))]
    start_time: String, // %Y-%m-%d %H:%M:%S
    #[serde(rename(deserialize = "dtEndTime"))]
    end_time: String, // %Y-%m-%d %H:%M:%S
    #[serde(rename(deserialize = "dtCreateTime"))]
    create_time: String, // %Y-%m-%d %H:%M:%S
    #[serde(rename(deserialize = "iInfoId"))]
    info_id: u32,
    #[serde(rename(deserialize = "sTagName"))]
    tag_name: Vec<String>,
    #[serde(rename(deserialize = "sCategoryName"))]
    category_name: String,
}

// TODO serde with; chrono/time; ureq/reqwest/hyper