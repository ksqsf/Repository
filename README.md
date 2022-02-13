# Repository

Repository is a personalized, open platform to save and query Web pages and associated resources.

Repository is not possible without awesome existing projects: youtube-dl, 

## Design and Concepts

The simplest accepted input is a URL, e.g. https://www.bilibili.com/video/av8888888 . This Web page is itself a resource.

Every resource is identified with a _Repository Resource Identifier_, which is a _stable_ counterpart of URL. For example, `bilibili:video:av12345` identifies a video found on Bilibili, and `bilibili:user:12345` identifies a user with UID 12345 on Bilibili. RRI is determined by the resource itself: even though bilibili changes its URLs frequently, this RRI should be always stable.

Therefore, for each URL, which identifies a Web Page resource, there are potentially many associated resources. For example, associated with https://www.bilibili.com/video/av8888888 is:

1. a web page snapshot
2. a video
3. danmaku
4. comment section

Each will be identified with its own RRI.

Finally, your Repository is a multigraph induced by the "associated" relationship, where each multinode represents a resource.

WIP: Each RRI can have potentially multiple versions, indexed by datetimes.

## License

GPLv3, unless stated otherwise.
