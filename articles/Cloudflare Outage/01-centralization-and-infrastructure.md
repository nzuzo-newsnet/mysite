#####
date = "2025-11-25"
author = "Nzuzo Magagula"
summary = "Exploring why one bug broke the internet, the role of centralization in modern tech, and how our consumption choices shape internet infrastructure"
thumbnail = "https://i.postimg.cc/3wMCP1N0/cracked-white-plaster-wall-texture-background.jpg"
category = "Opinion"
show_references = true

[[article_series]]
name = "Cloudflare Outage & Infrastructure Fragility"
next = ""

[[references]]
title = "Web3 - Wikipedia"
url = "https://en.wikipedia.org/wiki/Web3"
description = "Overview of Web3 principles and philosophies for decentralizing the internet"

[[references]]
title = "What Happened to Web3 - Slidebean"
url = "https://slidebean.com/story/what-happened-to-web3"
description = "Analysis of the Web3 startup boom and its sustainability challenges"

[[references]]
title = "The Post-Hype Playbook: Web3 Marketing Credibility - Hackernoon"
url = "https://hackernoon.com/the-post-hype-playbook-unhashed-ceo-mia-p-on-marketing-web3-credibility"
description = "How Web3 value propositions became lost in hype cycles"

[[references]]
title = "Web3 & Fintech 2025 and Beyond - LinkedIn"
url = "https://www.linkedin.com/pulse/web3-fintech-2025-beyond-making-waves-transforming-industries-jelic-2fhzf"
description = "Signs of Web3 evolving beyond hype as the ecosystem matures"

[[references]]
title = "Artificial Intelligence Illiteracy - The Atlantic"
url = "https://www.theatlantic.com/culture/archive/2025/06/artificial-intelligence-illiteracy/683021/?gift=a488bXrqvMlx1958JHI5qDnArF6wxd8fux6Y1VNDFMc"
description = "Parallels between AI and Web3 hype cycles and technical illiteracy"

[[references]]
title = "Is Cloudflare a Monopolist? - Dev.ua"
url = "https://dev.ua/en/news/chy-ie-cloudflare-monopolistom-iz-zakhystu-saitiv-vid-atak-ni-os-piatirka-alternatyv-1763473659"
description = "Analysis of Cloudflare's market position and available alternatives"

[[references]]
title = "Cloudflare is Destroying the Open Internet - GoAuthentik"
url = "https://version-2024-2.goauthentik.io/blog/2023-02-07-cloudflare-is-destroying-the-open-internet"
description = "Critical examination of Cloudflare's role in internet infrastructure sustainability"

[[references]]
title = "Cloudflare CEO on Google Abusing Monopoly - Fortune"
url = "https://fortune.com/2025/11/13/cloudflare-ceo-google-abusing-monopoly-search-ai/"
description = "Discussion of monopolistic practices and competitive behavior in tech"

[[references]]
title = "The Consumer Dilemma - SSRN"
url = "https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115"
description = "Academic paper on consumer choices against personal interests for societal benefit"

[[references]]
title = "AI Innovation and Monopolization - arXiv"
url = "https://arxiv.org/abs/2405.21015"
description = "Research on unsustainable innovation models designed to monopolize markets"

[[references]]
title = "Subscription Economics - ACM Digital Library"
url = "https://dl.acm.org/doi/abs/10.1145/3366423.3380281"
description = "Analysis of business models where consumers pay more than they consume"
#####
# The Fragility of Centralized Infrastructure: What the Cloudflare Outage Reveals About the Internet's Architecture

## Three Outages in a Month: What's Going On?

This feels like a good-faith question that's weirdly difficult to answer outright. We aren't all sitting in the offices of these service providers, watching their decision-making processes unfold. We can guess—probably guess pretty well—but unless an executive comes out and says "We all agreed to make things horrible," we're left making educated assumptions.

Personally, I think this question needs more than a simple answer. It's starting to feel like the infrastructure of the internet is becoming fundamentally fragile, not because of technical limitations, but because of *how we're organizing and delegating services*.

Think of it this way: imagine if three major cities all got their water from the same treatment plant, their electricity from the same power station, and their food from the same distribution center. When that treatment plant goes down, it's not just one neighborhood that loses water—it's three entire cities. The efficiency that made this centralization attractive in the first place becomes the exact mechanism of catastrophic failure.

That's essentially what's happening with internet infrastructure. And many articles have covered this in great detail, but I want to summarize the key issues and make a case of my own about why this keeps happening—and why it might be inevitable under current economic structures.

## Big Tech and the Inevitability of Concentration

It's no secret that we're living in an era where it's almost *expected* that every industry will eventually end up with a concentration of resources and market share, with a few companies becoming the "default" providers for essential services.

Cast your mind back a few years. A popular proposed solution to this problem was to adopt [Web3 principles and philosophies](https://en.wikipedia.org/wiki/Web3), which aimed to decentralize the internet. The vision was compelling: instead of a few massive companies controlling infrastructure, we'd have distributed networks where no single point of failure could bring everything down.

Unfortunately, as many promising movements do, Web3 was quickly treated as a new market in which one could become the monopoly *again*. Web3 startups were popping up [like crazy](https://slidebean.com/story/what-happened-to-web3), and it became painfully clear that this trend was not sustainable. The movement had become exactly what it claimed to oppose.

Many criticisms of Web3 in this era revolved around how apparent it was that these companies were in a gold rush. The actual value proposition of Web3 became lost in [mostly hype](https://hackernoon.com/the-post-hype-playbook-unhashed-ceo-mia-p-on-marketing-web3-credibility), though there are signs things might be [changing](https://www.linkedin.com/pulse/web3-fintech-2025-beyond-making-waves-transforming-industries-jelic-2fhzf) as the hype cycle settles and serious builders remain.

The point is: decentralization as a principle became difficult to understand amid the noise of hype cycles. Sound [familiar](https://www.theatlantic.com/culture/archive/2025/06/artificial-intelligence-illiteracy/683021/?gift=a488bXrqvMlx1958JHI5qDnArF6wxd8fux6Y1VNDFMc)? (Looking at you, current AI discourse.)

This is just one example of how the tech industry always seems to default into centralization. At this point, it feels inevitable.

### Why Centralization Keeps Winning

But here's the uncomfortable truth: I still think decentralization is a worthwhile pursuit. We don't need 1,001 different crypto startups that are essentially the same idea repeated ad nauseam. We also don't need companies and startups looking to dominate a specific corridor of Web3 infrastructure.

What we need is to fundamentally change how we approach centralization itself.

The nature of our economy has rewarded resource acquisition and scarcity creation to such an extent that you can almost guarantee who will be around in the next 10 years by measuring the depth of their pockets today. This is stark for end-user-facing services (like search engines or operating systems), but it affects the development community just as severely.

**Need a server?** There are about four main options, and if you *really* know what you want, you can find maybe an additional six providers worth considering.

**Need AI infrastructure?** You can count the number of reliable providers on your hands.

**Need DDoS protection and CDN services?** Well, there's Cloudflare, and then there's... well, technically there are alternatives, but are they *really* alternatives?

Similarly, it's difficult to imagine a tech economy where this concentration *isn't* the case, especially when everything has become a service. Infrastructure-as-a-Service, Platform-as-a-Service, Software-as-a-Service—the entire model incentivizes consolidation because economies of scale matter tremendously in infrastructure.

## The Cloudflare Paradox

The thing about Cloudflare is that it [isn't *really* a monopoly](https://dev.ua/en/news/chy-ie-cloudflare-monopolistom-iz-zakhystu-saitiv-vid-atak-ni-os-piatirka-alternatyv-1763473659)—but that's mostly a semantic argument. They have competitors, alternatives exist, and customers can technically switch. But they're growing concerningly large, and we should be [asking critical questions about their role in internet infrastructure and its long-term sustainability](https://version-2024-2.goauthentik.io/blog/2023-02-07-cloudflare-is-destroying-the-open-internet).

Here's where the problem gets genuinely weird, creating a paradox that's difficult to resolve:

**On one hand**, we want to know there's a service provider that's genuinely good at what they do. It's not like Cloudflare is actively [using their position to maintain their monopoly unfairly](https://fortune.com/2025/11/13/cloudflare-ceo-google-abusing-monopoly-search-ai/) through anticompetitive practices. They're popular for legitimate reasons: their services work well, they're often free for small sites, they're fast, and they've built trust through generally good engineering.

**On the other hand**, we need to assess these individual decisions in aggregate. When everyone independently decides that Cloudflare is the best option, we collectively create a dangerous single point of failure. And when we try to convince people to make different choices, we run into a fundamental problem of incentives.

### The Consumer Dilemma: Why Individual Rationality Creates Collective Risk

The [Consumer Dilemma](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115) describes a situation where consumers need to make consumption choices *against* their immediate personal interests in favor of greater societal interests.

Often, these are ethical considerations that feel more tangible and easier to engage with:
- "Should I avoid buying from companies with harmful production processes, even though it's more expensive?"
- "Should I boycott this company despite their products being superior, because of their extracurricular activities?"

These questions are relatively easy to engage with because the harm feels concrete and traceable. It's straightforward to understand why using more plastic contributes to environmental harm, or why supporting sweatshop labor perpetuates exploitation.

But this dilemma reveals itself in a peculiar way in the tech space.

### The Invisibility of Infrastructure Harm

Technology as a product is often so far removed from its societal effects that consumers are more comfortable consuming potentially harmful products because the harm is hidden or simply difficult to connect.

Even in instances where you can make the connection fairly [reasonably](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115), the benefits of new technology often *require* that consumers continue consuming or risk being left behind. This phenomenon is exacerbated when:

1. **The product in question is intangible** (a CDN service, a cloud platform)
2. **The harm in question is speculative** (a potential future outage)
3. **The harm is distant** (affecting "the internet" rather than "me specifically")
4. **The harm feels inconsequential to the individual consumer** (so what if everyone's site goes down together?)

Let me illustrate with a thought experiment:

Imagine you're choosing a hosting provider for your website. Provider A (let's call them BigCloud) has:
- 99.99% uptime
- Fast global performance
- Excellent documentation
- Free tier for small projects
- Used by 60% of the internet

Provider B (let's call them SmallCloud) has:
- 99.9% uptime
- Good regional performance
- Decent documentation
- No free tier
- Used by 2% of the internet

The rational choice for *you* as an individual is obvious: BigCloud is objectively better by almost every metric that matters to you right now. The fact that BigCloud already serves 60% of the internet doesn't make your website slower or less reliable—if anything, their scale is part of *why* they're so good.

But if everyone makes this same rational choice, we end up with 80% of the internet depending on one provider. And when that provider has an outage, 80% of the internet goes down.

The harm isn't visible in the individual choice. It only emerges in the aggregate.

### The Unconvincing Nature of Preventive Arguments

At some point, when a provider is doing things right, convenience becomes such a powerful pull—and oligarchical considerations feel so distant and abstract—that monopolies gradually emerge.

It's difficult to characterize this as inherently harmful because there are clearly genuine benefits pulling consumers (even outside of shady monopolistic practices). As a result, arguments against centralization end up sounding unconvincing:

> "I know that this is the best provider of Thing A right now, but don't use it because one day, if many people use this objectively better product, a monopoly *might* form and..."

And what? That's the complication I want to explore.

**The argument falls apart because:**

1. **The harm is hypothetical**: "Your service *might* go down in the future during an outage"
2. **The alternative is certain**: "If you use the inferior service, your site *will* be slower/less reliable *right now*"
3. **The collective harm doesn't feel personal**: "If Cloudflare goes down, everyone's site goes down, so I'm not uniquely disadvantaged"
4. **Historical performance contradicts the warning**: "Cloudflare has done a pretty good job of preventing major outages—better than most alternatives"

Let's dig into that last point, because it reveals something important about trust and monopolies.

## The Trust Problem: When Good Service Enables Dangerous Concentration

Consumers and service providers have an inherent trust relationship—one that requires a degree of faith in future performance based on past behavior.

It's difficult to claim that a company **will** become a bad monopoly because, what proof do we have? Cloudflare has been, by most accounts, a good actor. They provide free services to small sites, they publish detailed post-mortems when things go wrong, they've taken stands on free speech issues, and they've generally been transparent about their operations.

Similarly, it's difficult to make a convincing case about unintentional failure when:

1. **That risk isn't unique to monopolies**: Small providers have outages too, often more frequently
2. **The monopoly's track record is better**: Cloudflare's reliability is part of why they became so popular
3. **Individual customers have misaligned incentives**: If you're a Cloudflare customer, why would you care if "everyone's" service goes down? Your users won't blame you specifically—they'll understand it was a widespread outage

This creates a perverse dynamic: **The better a company is at providing service, the more we centralize around them. The more we centralize around them, the more catastrophic their inevitable failures become.**

### The Bathtub Analogy

Think of internet infrastructure like a bathtub. When you have many small providers (many small bathtubs), an individual outage is like one bathtub springing a leak—annoying for those using that specific bathtub, but everyone else continues normally.

When you centralize around one massive provider (one enormous bathtub), an outage is like that giant bathtub suddenly draining. The sheer volume of users affected turns a technical failure into a global event.

The paradox is that the giant bathtub is actually *better engineered* and *less likely* to spring a leak. It has better materials, better maintenance, more redundancy. But when it does fail—and all systems eventually fail—the consequences are orders of magnitude worse.

We've optimized for average-case performance at the expense of worst-case resilience.

## So What Should We Do About It?

Honestly? I'm not entirely sure. If I were, I'm not confident I could explain the solution well enough in this format without getting overly technical or proposing changes that require reimagining fundamental economic structures.

But I can share what drew me to Web3 in the first place, and what I still find valuable in that vision—even if the implementation got hijacked by speculation and hype.

### What I Hoped Web3 Would Be

The reason I was optimistic about Web3 was because I saw glimpses of a world that pivoted toward ownership and collaboration over rent-seeking and dependency.

I envisioned Web3 as an opportunity for:

- **More secure, resilient networks** where no single point of failure could cascade into global outages
- **Services that eventually inform how hardware is manufactured and sold**, prioritizing interoperability over lock-in
- **An AI market that makes innovation inherently costly** rather than pursuing innovation that intentionally reduces costs only to [monopolize the market later](https://arxiv.org/abs/2405.21015)

That last point deserves expansion. The current business model for many tech services involves:

1. Operating at a loss to gain market share (Amazon's strategy for years)
2. Undercutting competitors until they can't compete
3. Achieving dominance and then gradually increasing prices or reducing service quality
4. Leveraging network effects and switching costs to maintain position

This is the standard playbook, and it's economically rational under current structures. But it's also [clearly intending to monopolize markets](https://arxiv.org/abs/2405.21015), and we pretend to be surprised when it works.

### The Uncomfortable Reality of Digital Ownership

Here's what makes me genuinely uncomfortable:

Personal photos and videos of my life could one day be lost because of a missed payment to a cloud storage provider. The current business model for most services is designed to make consumers [pay more than they could ever consume](https://dl.acm.org/doi/abs/10.1145/3366423.3380281), extracting maximum revenue through subscriptions for resources that sit idle most of the time.

I don't own my data—I rent access to it, in perpetuity, from companies that can change terms, raise prices, or simply shut down services at will.

This isn't how we think about most important things in our lives. Imagine if your photo albums at home stopped working unless you paid a monthly subscription. Imagine if your bookshelf charged you rent per book, and if you stopped paying, all the books disappeared. It sounds absurd, but that's exactly the model we've accepted for digital goods.

### A Different Vision

I would love to reach a stage where our societal approach to new technology focuses on **accessibility** instead of **perpetual reliance and income extraction**.

What might this look like?

- **Infrastructure that's designed for resilience rather than efficiency alone**, accepting that some redundancy and "waste" is actually crucial for systemic health
- **Protocols over platforms**, where interoperability is the default rather than the exception
- **Real ownership of digital goods**, where buying something means actually owning it, not licensing access
- **Cooperative models for infrastructure**, where users have actual stake and governance in the services they depend on

But here's the challenge: these models often can't compete with VC-funded companies willing to operate at a loss for years to gain market dominance. The economic incentives are misaligned with the outcomes we actually want.

## The Path Forward: Questions We Need to Ask

I don't have complete answers, but I believe we need to start asking better questions:

### For Individual Users and Developers:

- **Can I reasonably diversify my infrastructure dependencies?** Even if Cloudflare is the best option, can I architect my systems so that an outage there doesn't completely destroy my service?
- **Am I making the convenient choice or the resilient choice?** Sometimes these align, but often they don't.
- **What am I willing to sacrifice for better systemic health?** Slightly slower performance? Higher costs? More complexity?

### For Service Providers:

- **Are we being transparent about our role in internet infrastructure?** Cloudflare deserves credit for their detailed post-mortems, but transparency alone doesn't solve the centralization problem.
- **Are we designing for graceful degradation?** When our services fail—and they will—do they fail catastrophically or do they fail gracefully with partial functionality?
- **What responsibilities come with our market position?** If you serve 60% of the internet's traffic, you're no longer just a company—you're infrastructure. What obligations does that create?

### For Society and Policymakers:

- **Should critical internet infrastructure be treated differently from other services?** We regulate utilities, we regulate telecommunications. Should we regulate internet infrastructure providers differently?
- **How do we incentivize resilience over efficiency?** Markets naturally optimize for efficiency, but resilience often looks like waste—until it isn't.
- **What does meaningful decentralization actually look like?** Not 1,001 cryptocurrency startups, but genuinely distributed infrastructure that can withstand localized failures.

## Conclusion: Living with Fragility

The Cloudflare outage—and the three others this month—aren't anomalies. They're symptoms of an internet architecture that has optimized itself toward centralization because that's what all the incentives pointed toward.

We've built an incredibly efficient system that delivers blazing-fast performance and remarkable reliability 99.9% of the time. But we've done so by concentrating risk in ways that make the 0.1% of failures catastrophic.

The uncomfortable truth is that this might be exactly what we collectively chose, through millions of individual rational decisions that created an irrational whole. We chose convenience, performance, and cost-effectiveness. We got all three, and the bill comes due in the form of occasional internet-wide outages.

Whether we're willing to pay a different price—in performance, in convenience, in complexity—for a more resilient internet remains an open question.

But it's a question we need to start asking seriously, because the alternative is continuing to act surprised when the same thing keeps happening.

The internet didn't crash because of a `.unwrap()` call in Rust code. It crashed because we've built a system where a single company's mistake can break the world's infrastructure. The code bug was just the trigger. The architecture is the weapon.

And we loaded it ourselves.
