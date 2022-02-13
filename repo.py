# WARNING: This is currently an experimental proof-of-concept of Repository.

import subprocess
import queue
import urllib.parse

class Handler(object):
    def __init__(self, name):
        self.name = name
    
    def can_handle(self, rri: str):
        """
        Returns True if this RRI can by handled by this handler, False otherwise.
        """
        return False
    
    def local_path(self, rri: str):
        """
        Returns an appropriate local path computed from the RRI.
        """
        raise NotImplementedError()

    def try_archive(self, rri: str):
        """
        Returns True if archived successfully, False otherwise.
        """
        return False

    def discover(self, rri: str) -> list[str]:
        """
        Find whether this RRI can lead to more RRIs.
        """
        return []


class WebPage(Handler):
    def __init__(self):
        super().__init__('WebPage')

    def can_handle(self, rri: str):
        rri = rri.lower()
        return rri.startswith("http://") or rri.startswith("https://")

    def local_path(self, rri: str):
        return "/tmp/1.mhtml"

    def try_archive(self, rri: str):
        print('todo(webpage): archive rri: ' + rri)
        pass


class BilibiliVideo(Handler):
    def __init__(self):
        super().__init__('BilibiliVideo')

    def can_handle(self, rri: str):
        rri = rri.lower()
        return rri.startswith('bilibili:video:')

    def _normalize(self, rri: str):
        return rri[len('bilibili:video:'):]

    def local_path(self, rri: str):
        return '/tmp/' + self._normalize(rri) + '.mp4'
    
    def try_archive(self, rri: str):
        url = 'http://bilibili.com/video/' + self._normalize(rri)
        subprocess.call(['youtube-dl', url, '--output', self.local_path(rri)])
        return True

    def discover(self, upper_rri: str):
        res = urllib.parse.urlparse(upper_rri)
        if res.scheme in ['http', 'https']:
            return ['bilibili:video:' + res.path.split('/')[-1]]
        else:
            return []


HANDLERS = [
    WebPage(),
    BilibiliVideo(),
]


DATABASE = dict()


class Resource:
    def __new__(cls, rri):
        if rri not in DATABASE:
            DATABASE[rri] = object.__new__(cls)
        return DATABASE[rri]

    def __init__(self, rri):
        self.rri = rri
        self.associated = set()

    def __str__(self):
        return self.rri

    def __repr__(self):
        return f'<Resource {self.rri}>'

    def link(self, child_rri):
        self.associated.add(Resource(child_rri))

    def archive(self):
        """
        Archive this resource.
        """
        global HANDLERS
        resources = queue.Queue()
        resources.put(self.rri)
        while not resources.empty():
            cur = resources.get()
            for handler in HANDLERS:
                if handler.can_handle(cur):
                    handler.try_archive(cur)
                if assocs := handler.discover(cur):
                    for assoc in assocs:
                        resources.put(assoc)
                        Resource(cur).associated.add(Resource(assoc))
                        print(cur + ' <- ' + assoc)
        return resources

Resource('https://www.bilibili.com/video/BV1gU4y187xA').archive()

def pr(res, d=0):
    print('\t' * d, end='')
    print(res.rri)
    for assoc in res.associated:
        pr(assoc, d+1)
for res in DATABASE.values():
    pr(res)
