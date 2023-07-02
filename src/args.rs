use clap::Parser;
use lazy_static::lazy_static;
use std::convert::Infallible;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, env = "MICROBIN_BASIC_AUTH_USERNAME")]
    pub auth_basic_username: Option<String>,

    #[clap(long, env = "MICROBIN_BASIC_AUTH_PASSWORD")]
    pub auth_basic_password: Option<String>,

    #[clap(long, env = "MICROBIN_ADMIN_USERNAME", default_value = "admin")]
    pub auth_admin_username: String,

    #[clap(long, env = "MICROBIN_ADMIN_PASSWORD", default_value = "m1cr0b1n")]
    pub auth_admin_password: String,

    #[clap(long, env = "MICROBIN_EDITABLE")]
    pub editable: bool,

    #[clap(long, env = "MICROBIN_FOOTER_TEXT")]
    pub footer_text: Option<String>,

    #[clap(long, env = "MICROBIN_HIDE_FOOTER")]
    pub hide_footer: bool,

    #[clap(long, env = "MICROBIN_HIDE_HEADER")]
    pub hide_header: bool,

    #[clap(long, env = "MICROBIN_HIDE_LOGO")]
    pub hide_logo: bool,

    #[clap(long, env = "MICROBIN_NO_LISTING")]
    pub no_listing: bool,

    #[clap(long, env = "MICROBIN_HIGHLIGHTSYNTAX")]
    pub highlightsyntax: bool,

    #[clap(short, long, env = "MICROBIN_PORT", default_value_t = 8080)]
    pub port: u16,

    #[clap(short, long, env="MICROBIN_BIND", default_value_t = IpAddr::from([0, 0, 0, 0]))]
    pub bind: IpAddr,

    #[clap(long, env = "MICROBIN_PRIVATE")]
    pub private: bool,

    #[clap(long, env = "MICROBIN_PURE_HTML")]
    pub pure_html: bool,

    #[clap(long, env = "MICROBIN_JSON_DB")]
    pub json_db: bool,

    #[clap(long, env = "MICROBIN_PUBLIC_PATH")]
    pub public_path: Option<PublicUrl>,

    #[clap(long, env = "MICROBIN_SHORT_PATH")]
    pub short_path: Option<PublicUrl>,

    #[clap(long, env = "MICROBIN_READONLY")]
    pub readonly: bool,

    #[clap(long, env = "MICROBIN_SHOW_READ_STATS")]
    pub show_read_stats: bool,

    #[clap(long, env = "MICROBIN_TITLE")]
    pub title: Option<String>,

    #[clap(short, long, env = "MICROBIN_THREADS", default_value_t = 1)]
    pub threads: u8,

    #[clap(short, long, env = "MICROBIN_GC_DAYS", default_value_t = 90)]
    pub gc_days: u16,

    #[clap(long, env = "MICROBIN_ENABLE_BURN_AFTER")]
    pub enable_burn_after: bool,

    #[clap(short, long, env = "MICROBIN_DEFAULT_BURN_AFTER", default_value_t = 0)]
    pub default_burn_after: u16,

    #[clap(long, env = "MICROBIN_WIDE")]
    pub wide: bool,

    #[clap(long, env = "MICROBIN_QR")]
    pub qr: bool,

    #[clap(long, env = "MICROBIN_ETERNAL_PASTA")]
    pub eternal_pasta: bool,

    #[clap(long, env = "MICROBIN_ENABLE_READONLY")]
    pub enable_readonly: bool,

    #[clap(long, env = "MICROBIN_DEFAULT_EXPIRY", default_value = "24hour")]
    pub default_expiry: String,

    #[clap(short, long, env = "MICROBIN_NO_FILE_UPLOAD")]
    pub no_file_upload: bool,

    #[clap(long, env = "MICROBIN_CUSTOM_CSS")]
    pub custom_css: Option<String>,

    #[clap(long, env = "MICROBIN_HASH_IDS")]
    pub hash_ids: bool,

    #[clap(long, env = "MICROBIN_ENCRYPTION_CLIENT_SIDE")]
    pub encryption_client_side: bool,

    #[clap(long, env = "MICROBIN_ENCRYPTION_SERVER_SIDE")]
    pub encryption_server_side: bool,

    #[clap(
        long,
        env = "MICROBIN_MAX_FILE_SIZE_ENCRYPTED_MB",
        default_value_t = 256
    )]
    pub max_file_size_encrypted_mb: usize,

    #[clap(
        long,
        env = "MICROBIN_MAX_FILE_SIZE_UNENCRYPTED_MB",
        default_value_t = 2048
    )]
    pub max_file_size_unencrypted_mb: usize,
}

impl Args {
    pub fn public_path_as_str(&self) -> String {
        if self.public_path.is_some() {
            self.public_path.as_ref().unwrap().to_string()
        } else {
            String::from("")
        }
    }

    pub fn short_path_as_str(&self) -> String {
        if self.short_path.is_some() {
            self.short_path.as_ref().unwrap().to_string()
        } else if self.public_path.is_some() {
            self.public_path.as_ref().unwrap().to_string()
        } else {
            String::from("")
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublicUrl(pub String);

impl fmt::Display for PublicUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PublicUrl {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = s.strip_suffix('/').unwrap_or(s).to_owned();
        Ok(PublicUrl(uri))
    }
}
