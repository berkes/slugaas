# Spicy-Scarlet-Slug

A tiny (🌶️), selfhosted(💜) web-service to generate unique slugs (🐌).

Written in *rust*, to make it easy to run and deploy. 

I Wrote this as a sideproject to learn Axum, and to gain experience with
hosting and CI for rust pipeline.

```
GET http://0.0.0.0:3000/suggestion
HTTP/1.1 200

GET http://0.0.0.0:3000/suggestion?seed=foo
"unique-crepe-shark"

POST http://0.0.0.0:3000/registrations
slug: "angry-purple-tiger"
HTTP/1.1 202

GET http://0.0.0.0:3000/registrations?slug=angry-purple-tiger
HTTP/1.1 200

GET http://0.0.0.0:3000/registrations?slug=fancy-blue-monkey
HTTP/1.1 404
```
See [the hurl file](tests/suggstions.hurl) for more details.

A common workflow is to:

1. Generate a slug client-side, OR
1. Have the customer type one in, OR
1. Get one from the service at /suggestions
1. Search the service whether the provided suggestion is in use already.
1. Do something with it - e.g. create a new blogpost, container, project etc.
1. Register this slug at the service. So it cannot be re-used.

The last step will return an error if the slug you want to store was already registered.
This avoids human-generated slugs being reused. And it avoids race-conditions
where between searching & registering another service registered your slug.

Once registered, this slug will no longer show up in suggestions.

Checking for registrations should be fast enough to be used client-side. E.g.
a textbox that shows a validation checkmark when someone stops typing. It is 
not a type-ahead search, as it will only return slugs with an exact match.
It is not a search at all, since that would expose slugs in use.

Slugs are append-only by nature. You cannot change or delete any from the registry.

## Security 

This is *not*  a security tool. It is *probably not* random enough to work as
password generator, or any kind of secret. Slugs are meant as public data,
and should be treated as such.

## Angry Purple Tiger

It leverages [angry-purple-tiger](https://github.com/helium/angry-purple-tiger-rs)
to generate *suggestions*. 

## HTTP

We use [Axum](https://docs.rs/axum/latest/axum/) as HTTP service.

## storage

TODO: or rdps?

We use [PickleDB](https://github.com/seladb/pickledb-rs) as storage. There
may be better suited strategies for storing and lookup, so feel free to suggest
any. PickleDB is probably not the fastest or leanest. It reads an entire database
in memory, so it doesn't scale with lots of slugs. ~~The namespaces are implemented using
the value of the KV storage, which feels hacky~~.
Any alternatives would, like PickleDB, need to support appending, be
thread-safe, have fast lookups, ~~and support some mechanism to filter by namespace.~~

## Tools

TODO: Add tools and write this.

## Quickstart

TODO: write this.

### Install
TODO: implement this. Below does not work.

In order to install the platform on development machine, run

    make install

This installs and configures the dependencies.


### Run
TODO: implement this. Below does not work.

After installing the dependencies, on the development machine, run

    make

This builds and runs the platform locally.

### Test

TODO: implement this. 

After installing the dependencies, on the development machine, run


    make test

This builds and runs **all** tests locally.

#### Integration tests

We test the API through `hurl`.

```bash
cargo run
hurl --test test/*hurl
```

This starts the service, then runs the hurl tests.

### Release
TODO: implement this. Below does not work.

After finishing the changes, a release can be prepared with

    make release

This uses amongst others, git-flow to create and tag a new release. It
bumps the version number. Add `VERSION=x.y.z-special` to set a specific
version, instead of incrementing the next minor version (1.2.3 -> 1.3.0)

### Deploy

After running a build, testing successfully, one can deploy with

    make deploy

This checks preconditions such as proper git-tags, branches, permissions
and sanity checks and when met, deploys current release.

