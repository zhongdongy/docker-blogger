from .preamble import Preamble


class TagIndex(object):
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
    tag_dict: dict[str, list[TagIndex]] = dict()

    def __init__(self):
        self.tag_dict = dict()

    def __len__(self):
        return self.length

    @property
    def length(self):
        return len(self.tag_dict)

    def add(self, tag: TagIndex):
        if tag.tag not in self.tag_dict:
            self.tag_dict[tag.tag] = list()
        self.tag_dict[tag.tag].append(tag)

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
