import hashlib


def get_gravatar_url(email: str) -> str:
    md5 = hashlib.md5()
    md5.update(email.lower().encode('utf-8'))
    return f"https://www.gravatar.com/avatar/{md5.hexdigest()}"
