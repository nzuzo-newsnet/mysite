Over the weekend, Cloudflare has an outage that essentially rendered most of the internet unusable. It got so bad that [even downdetector went down](https://www.msn.com/en-us/news/technology/the-internet-crashed-so-hard-this-morning-that-downdetector-went-down/ar-AA1QFyNB).
As is tradition, everyone had an opinion on this event and quite frankly, so do I. The thing is, my echo chamber seemed to ring the loudest about 2 main issues; The weird frequency of large service providers grinding the planet to a halt, and the roles and promises of Rust.
Both of these conversations were surprisingly polar and this discourse served as a welcome reminder that no matter how technical an industry, people are still people and our biases shape the conversations we have. It was interesting to see how emotional people got about something as trivial as a programming language and this insight has made me reasses a lot of how I approach software development in general.

# What was the problem?
In a [post mortum](https://blog.cloudflare.com/18-november-2025-outage/) published by Cloudflare, they reveal that a bug in their memory allocation validation didn't handle memory allocation limits properly.
I won't rewrite the blog here, but in essence, there was a funcitonality that allowed for machine learning features to be added to their Bot Management system, and to improve performance, they had a limit on the number of features.

In the code snippet below, you will see how the function adds features to some Features object with a method, which was fallable (Could successfully return an `Ok(output)` or an `Err(error)` variant, depending on the outcome of the call).

If you have been coding in rust for a bit, you would notice that it is weird to make an `.unwrap()` call in a production codebase. This function essentially panics if the output of the `append_with_names()` is an `Err`.
Often, `unwrap()` is used in testing and development environements, where it can be tedious thinking of how to handle *every* single error (or `None` variant) instance gracefully.
Well, as you can see, it was used in prod. This panics the `fetch_features` function and I'm sure you can guess the rest. The [post mortum](https://blog.cloudflare.com/18-november-2025-outage/) explains how this cascaded and lead to the widespread outage.

## How did people respond?
This is a bit of a complicated question to answer downright, because there are a number of layers and conversations that we being had at the same time. In particular two main question caught my attention.
Firstly, people want to know [why on earth are we seeing so many outages lately](https://www.nbcnews.com/tech/internet/internet-outages-aws-microsoft-cloudflare-rcna245043) and secondly, [wasn't rust supposed to fix this kind of stuff?](https://lucisqr.substack.com/p/the-cloudflare-outage-and-the-rust)
I think that both of these questions are fair to ask, but I don't know if everyone was asking them the same way.

# This is the [third outage in a month](https://edition.cnn.com/2025/11/18/tech/cloudflare-down-outage-cause), what's going on?
This feels like a good faith question that is a little weird to answer outright. We aren't all in the offices of these service providers, and we don't know what they are doing. We can guess, and probably guess pretty well, but unless an exec comes out and says "We all agreed to make things horrible", we can only make some fair guesses.
Personally, I think that this is a question that needs more than an answer. It is starting to fell like the infrastructure of the internet is becoming fragile because of the way we are organising and delegating services.
Many articles cover this in great detail, and I just wanted to summarise and make a case of my own.

## Big tech and popularity
It is no secret that we are moving into an era where it is almost *expected* that every industry will eventally end up with a concentration of resources and market share, becoming the "default" providers for many popular services.
If you can cast you mind back a few years, a popular solution to this problem was to adopt [Web 3 principles and philosophies](https://en.wikipedia.org/wiki/Web3), which aim to decentralise the internet.
Unfortunately, as many things to, this movement was treated as a new market in which one can become the monopoly again. Web3 startup were popping up [like crazy](https://slidebean.com/story/what-happened-to-web3) and it became clear that this trend was not sustainable.
Many criticisms of Web3 in this era mainly revolved around how it was apparent that these companies were in a gold rush and the value in Web 3 was [mostly hype](https://hackernoon.com/the-post-hype-playbook-unhashed-ceo-mia-p-on-marketing-web3-credibility), although it looks like things might be changing.
The point is, decentralization in principle became difficult to understand in the noise of the hype cycles. Sounds familiar?
This is just one example of how the tech industry always seems to default into centralisation and it feels inevitable.

I still think that this is a worthwhile persuit. We don't need 1001 different crypto startups that are essentially the same idea over and over again. We also don't need companies and startups that are looking to dominate a corridor of Web 3. We just need to change how we approach centralization.
The nature of the economy as rewarded resource acquisition and scarcity to such an extent that you can almost guarantee who will be around in the next 10 year by how big their pockets are. This is pretty stark for end user facing services (like search engines or Operating systems), but affects the development community as well.
Similarily, it is difficult to imagine a tech economy where this is not the case, especially when everything is a service now.
Need a server? There are about 4 main options, and if you *really* know what you want, you can find like an additional 6.
Need an AI? You can count the number of relaible providers on your hands.
The thing about Cloudflare, is that it [isn't *really* a monopoly](https://dev.ua/en/news/chy-ie-cloudflare-monopolistom-iz-zakhystu-saitiv-vid-atak-ni-os-piatirka-alternatyv-1763473659), but that is mostly a semantic argument.
They are growing concerningly large and we should be [asking questions about their role in the internet infrastructure and its sustainability](https://version-2024-2.goauthentik.io/blog/2023-02-07-cloudflare-is-destroying-the-open-internet).

See the problem is weird. On the one hand, we would like to know that there is a service provider that is good at what they do. It isn't like Cloudflare is actively [using their position to maintain their monopoly unfairly](https://fortune.com/2025/11/13/cloudflare-ceo-google-abusing-monopoly-search-ai/), they are popular for a reason.
On the other hand, we need to assess these decisions en mass, and when we do that it gets a bit tricky to assure that everyone decides "correcly".
