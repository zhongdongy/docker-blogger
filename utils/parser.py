import re
from datetime import datetime, date


def parse_datetime_string(datetime_str: str):
    if isinstance(datetime_str, date):
        return datetime.combine(datetime_str, datetime.now().time())
    elif isinstance(datetime_str, datetime):
        return datetime_str
    if datetime_str is None or len(datetime_str.strip()) == 0:
        return None
    datetime_str = datetime_str.strip()
    if re.match(r'^\d{4}-\d{2}-\d{2}$', datetime_str):
        return datetime.strptime(datetime_str, '%Y-%m-%d')
    if re.match(r'^\d{2}-\d{2}$', datetime_str):
        return datetime.strptime(datetime_str, '%m-%d')
    if re.match(r'^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$', datetime_str):
        return datetime.strptime(datetime_str, '%Y-%m-%d %H:%M')
    if re.match(r'^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$', datetime_str):
        return datetime.strptime(datetime_str, '%Y-%m-%d %H:%M:%S')
    else:
        return datetime.now()
