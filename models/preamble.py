from datetime import datetime

from utils.parser import parse_datetime_string


class Preamble(object):
    title: str = ''
    author: str = ''
    created_at: datetime = None
    updated_at: datetime = None
    tags: list[str] = list()
    keywords: list[str] = list()
    permanent_link: str | None = None
    description: str | None = None
    renderer_params: list[str]  = list()

    def __init__(self, raw: dict = None):
        if isinstance(raw, Preamble):
            self.title = raw.title
            self.author = raw.author
            self.created_at = raw.created_at
            self.updated_at = raw.updated_at
            self.permanent_link = raw.permanent_link
            self.tags = raw.tags
            self.keywords = raw.keywords
            self.description = raw.description
            self.renderer_params = raw.renderer_params
        else:
            self.title = (raw or {}).get('title')
            self.author = (raw or {}).get('author')
            self.created_at = parse_datetime_string((raw or {}).get('created_at'))
            self.updated_at = parse_datetime_string((raw or {}).get('updated_at'))
            self.permanent_link = (raw or {}).get('permanent_link')
            self.tags = (raw or {}).get('tags') or list()
            self.renderer_params = (raw or {}).get('renderer_params') or list()
            self.keywords = (raw or {}).get('keywords')
            self.description = (raw or {}).get('description')

    def to_dict(self):
        return dict(
            title=self.title,
            author=self.author,
            created_at=self.created_at.strftime('%Y-%m-%d %H:%M:%S'),
            updated_at=self.updated_at.strftime('%Y-%m-%d %H:%M:%S'),
            tags=self.tags,
            renderer_params=self.renderer_params,
            keywords=self.keywords,
            permanent_link=self.permanent_link,
            description=self.description,
        )
