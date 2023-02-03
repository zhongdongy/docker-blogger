from .preamble import Preamble


class PostIndex(object):
    tag: str
    name: str
    preamble: Preamble

    def __init__(self, raw: dict = None):
        self.tag = raw.get('tag')
        self.name = raw.get('name')
        self.preamble = Preamble(raw.get('preamble'))

    def to_dict(self):
        return dict(
            tag=self.tag,
            name=self.name,
            preamble=self.preamble.to_dict()
        )

class PermanentLinkIndex(object):
    perm_link: str
    name: str
    preamble: Preamble

    def __init__(self, raw: dict = None):
        self.perm_link = raw.get('permanent_link')
        self.name = raw.get('name')
        self.preamble = Preamble(raw.get('preamble'))

    def to_dict(self):
        return dict(
            tag=self.perm_link,
            name=self.name,
            preamble=self.preamble.to_dict()
        )


class TagIndexCollection:
    tag_dict: dict[str, list[PostIndex]] = dict()

    def __init__(self, raw: dict = None):
        self.tag_dict = dict()
        if raw is not None:
            for tag in raw:
                if tag not in self.tag_dict:
                    self.tag_dict[tag] = list()
                for tag_index in raw[tag]:
                    self.tag_dict[tag].append(PostIndex(tag_index))

    def posts(self, tag_name: str):
        if tag_name in self.tag_dict:
            return self.tag_dict.get(tag_name)
        return []

    @property
    def tags(self):
        return list(self.tag_dict.keys())

    def __len__(self):
        return self.length

    @property
    def length(self):
        return len(self.tag_dict)

    def add(self, post: PostIndex):
        if post.tag not in self.tag_dict:
            self.tag_dict[post.tag] = list()
        exists = False
        for existing_post in self.tag_dict[post.tag]:
            if post.name == existing_post.name:
                exists = True
                break
        if exists is False:
            self.tag_dict[post.tag].append(post)

    def to_dict(self):
        result = dict()

        for t in self.tag_dict:
            result[t] = list(map(lambda tag: tag.to_dict(), self.tag_dict[t]))

        return result


class PermanentLinkIndexCollection:
    perm_link_dict: dict[str, PermanentLinkIndex] = dict()

    def __init__(self):
        self.perm_link_dict = dict()

    def __len__(self):
        return self.length

    @property
    def length(self):
        return len(self.perm_link_dict)

    def add(self, link: PermanentLinkIndex):
        self.perm_link_dict[link.perm_link] = link

    def to_dict(self):
        result = dict()

        for link in self.perm_link_dict:
            result[link] = self.perm_link_dict[link].to_dict()

        return result
