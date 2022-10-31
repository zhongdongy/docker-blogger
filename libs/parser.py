import re
from html.parser import HTMLParser

import yaml

from models.preamble import Preamble

preamble_regex = re.compile(r'^---([\s\S]+)---')


def parse_preamble(content: str) -> [Preamble, str]:
    preamble_match = preamble_regex.match(content)
    if preamble_match is None:
        return Preamble(), content

    preamble_str = preamble_match[1]
    preamble = yaml.load(preamble_str, yaml.Loader)
    return Preamble(preamble), content.replace(f"---{preamble_str}---", '')


class HeadingParser(HTMLParser):
    heading_ids = list()
    start_parsing = 0
    get_content = False

    def __init__(self):
        super().__init__(convert_charrefs=False)
        self.heading_ids = list()
        self.start_parsing = 0
        self.get_content = False

    def handle_starttag(self, tag, attrs):
        if self.start_parsing > 0:
            self.start_parsing += 1

        if self.start_parsing > 0 and tag in ['h1', 'h2', 'h3']:
            for tup in attrs:
                if tup[0] == 'id':
                    self.heading_ids.append(dict(
                        tag=tag,
                        id=tup[1]
                    ))
                    self.get_content = True
                    break
        if tag == 'div':
            for tup in attrs:
                if tup[0] == 'class' and 'post-content' in tup[1]:
                    self.start_parsing = 1

    def handle_data(self, data: str) -> None:
        if self.get_content is True:
            self.heading_ids[-1]['content'] = data
            self.get_content = False

    def handle_endtag(self, tag: str) -> None:
        if self.start_parsing > 0:
            self.start_parsing -= 1


def parse_headings(html: str) -> list:
    parser = HeadingParser()
    parser.feed(html)
    heading_ids = parser.heading_ids
    parser.close()
    return heading_ids

# def parse_headings(markdown: str)->list:
#     headings = list()
#     for line in markdown.split('\n'):
#         line = line.strip()
#         if re.match(r'^#+\s',line):
