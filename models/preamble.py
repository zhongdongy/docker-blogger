from datetime import datetime

from utils.parser import parse_datetime_string


class Preamble(object):
    title: str = ''
    author: str = ''
    author_email: str = ''
    created_at: datetime = None
    updated_at: datetime = None
    allow_before_epoch = False
    tags: list[str] = list()
    keywords: list[str] = list()
    permanent_link: str | None = None
    description: str | None = None
    renderer_params: list[str] = list()
    redirect: str | None = None
    author_avatar: str | None = None

    def __init__(self, raw: dict = None):
        if isinstance(raw, Preamble):
            self.title = raw.title
            self.author = raw.author
            self.author_email = raw.author_email
            self.created_at = raw.created_at
            self.updated_at = raw.updated_at
            self.permanent_link = raw.permanent_link
            self.tags = raw.tags
            self.keywords = raw.keywords
            self.description = raw.description
            self.renderer_params = raw.renderer_params
            self.redirect = raw.redirect
            self.author_avatar = raw.author_avatar
            self.allow_before_epoch = raw.allow_before_epoch
        else:
            self.title = (raw or {}).get('title')
            self.author = (raw or {}).get('author')
            self.author_email = (raw or {}).get('author_email')
            self.allow_before_epoch = (raw or {}).get('allow_before_epoch')
            self.created_at = parse_datetime_string((raw or {}).get('created_at'))
            self.updated_at = parse_datetime_string((raw or {}).get('updated_at'))
            self.permanent_link = (raw or {}).get('permanent_link')
            self.tags = (raw or {}).get('tags') or list()
            self.renderer_params = (raw or {}).get('renderer_params') or list()
            self.keywords = (raw or {}).get('keywords')
            self.description = (raw or {}).get('description')
            self.redirect = (raw or {}).get('redirect')
            self.author_avatar = (raw or {}).get('author_avatar')

    def to_dict(self):
        return dict(
            title=self.title,
            author=self.author,
            author_email=self.author_email,
            created_at=self.created_at.strftime('%Y-%m-%d'),
            updated_at=self.updated_at.strftime('%Y-%m-%d'),
            tags=self.tags,
            renderer_params=self.renderer_params,
            keywords=self.keywords,
            permanent_link=self.permanent_link,
            description=self.description,
            redirect=self.redirect,
            author_avatar=self.author_avatar,
            allow_before_epoch=self.allow_before_epoch,
        )
