import hashlib
from utils.config import load_config


def get_gravatar_url(email: str) -> str:
    md5 = hashlib.md5()
    md5.update(email.lower().encode('utf-8'))
    config = load_config()
    gravatar_url = "https://www.gravatar.com/avatar/"
    if config.site.gravatar_proxy is not None and len(config.site.gravatar_proxy) > 0:
        gravatar_url = config.site.gravatar_proxy
    if not gravatar_url.endswith('/'):
        gravatar_url += '/'
    return f"{gravatar_url}{md5.hexdigest()}"
