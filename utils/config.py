import yaml


class Public(object):
    home_post: str

    def __init__(self, raw: dict = None):
        self.home_post = (raw or {}).get('home_post')


class Beian(object):
    icp_id: str
    beian_id: str
    enabled: bool = False

    def __init__(self, raw: dict = None):
        self.icp_id = (raw or {}).get('icp_id')
        self.beian_id = (raw or {}).get('beian_id')
        self.enabled = (raw or {}).get('enabled') or False


class Site(object):
    beian: Beian
    hostname: str
    enable_https = False
    site_name: str = "那阵东风"
    site_email: str = "zhongdongy@dongs.xyz"
    site_slogan: str = '由 <a href="https://hub.docker.com/r/dongsxyz/blogger" target="_blank">Eastwind Blogger</a> 驱动'
    gravatar_proxy: str | None = None

    def __init__(self, raw: dict = None):
        self.beian = Beian((raw or {}).get('beian'))
        self.hostname = (raw or {}).get('hostname') or "127.0.0.1:5000"
        self.enable_https = (raw or {}).get('enable_https') or False
        self.site_name = (raw or {}).get('site_name') or "那阵东风"
        self.site_email = (raw or {}).get('site_email') or "zhongdongy@dongs.xyz"
        self.site_slogan = (raw or {}).get('site_slogan') or ('由 <a href="https://hub.docker.com/r/dongsxyz/blogger" '
                                                              'target="_blank">Eastwind Blogger</a> 驱动')
        self.gravatar_proxy = (raw or {}).get('gravatar_proxy') or None


class AppConfig(object):
    public: Public
    site: Site

    def __init__(self, raw: dict = None):
        self.public = Public((raw or {}).get('public'))
        self.site = Site((raw or {}).get('site'))


def load_config():
    with open('config.yml', 'r', encoding='utf-8') as config_content:
        return AppConfig(yaml.safe_load(config_content))
