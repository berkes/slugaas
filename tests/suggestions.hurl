GET http://0.0.0.0:3000/suggestion
HTTP/1.1 200
# Cannot assert body, as it is random.

GET http://0.0.0.0:3000/suggestion?seed=foo
HTTP/1.1 200
[Asserts]
body == "unique-crepe-shark"

POST http://0.0.0.0:3000/registrations
[FormParams]
slug: angry-purple-tiger
HTTP/1.1 202

GET http://0.0.0.0:3000/registrations?slug=angry-purple-tiger
HTTP/1.1 200

GET http://0.0.0.0:3000/registrations?slug=fancy-blue-monkey
HTTP/1.1 404
