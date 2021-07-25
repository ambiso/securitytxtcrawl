# Security.txt Crawler

This application will download the `/.well-known/security.txt` of a list of domains.

## Usage

To download a list of top 1 million websites and crawl them:

```bash
$ cargo run --release
```

The results are saved in `output/`.

```bash
$ cargo run --release -- --help

securitytxtcrawl 0.1.0
Crawl the security.txt of many websites

USAGE:
    securitytxtcrawl [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <concurrency>    Set the number of concurrent requests to perform [default: 200]
    -d, --domains <domains-fname>      Filename of the CSV containing the domains to crawl The CSV should contain a
                                       numeric ID and the domain. There is no header. If the file does not exist, a top
                                       1 million domains list is downloaded [default: 1m.csv]
    -o, --outdir <output>              Output directory for security.txts. A file containing the contents of the
                                       security.txt is created for each domain [default: output]
```

## Why?

Now you can find out who's hiring, for example:

```bash
$ grep -rn '^Hiring: ' output
shopify.com.co:6:Hiring: https://www.shopify.com/careers                                   
google.cz:6:Hiring: https://g.co/SecurityPrivacyEngJobs                
duo.com:4:Hiring: https://duo.com/about/careers
jira.com:12:Hiring: https://www.atlassian.com/company/careers/all-jobs?team=Security
[...]
```

To get a list of all hiring URLs:

```
$ grep -hr '^Hiring: ' | sort | uniq > hiring.txt
```

<details>
<summary>All Hiring URLs</summary>

```
Hiring: 
Hiring: 
Hiring: /careers//
Hiring: careers.michelin.com
Hiring: careers.swapcard.com
Hiring: hhttps://www.gitpod.io/careers
Hiring: https://99designs.com/about/jobs
Hiring: https://ably.com/careers
Hiring: https://about.crunchbase.com/about-us/careers
Hiring: https://acrisure.com/about-acrisure/careers/
Hiring: https://aida.de/careers
Hiring: https://aircall.io/careers/
Hiring: https://airtable.com/jobs
Hiring: https://allegro.tech/jobs/
Hiring: https://athena.bamboohr.com/jobs/
Hiring: https://auspost.com.au/jobs
Hiring: https://auth0.com/careers/positions?areas=Security
Hiring: https://blog.grabcad.com/jobs/
Hiring: https://blog.mention-me.com/careers
Hiring: https://bloomscape.com/careers/
Hiring: https://boards.greenhouse.io/intercom/jobs/1888872
Hiring: https://boards.greenhouse.io/reddit
Hiring: https://bowtie.bamboohr.com/jobs/
Hiring: https://bsg.tech/contact
Hiring: https://burberrycareers.com/search/?q=security 
Hiring: https://careers.aegon.com/en/vacancies/?query=security
Hiring: https://career.sansiri.com
Hiring: https://careers.dazn.com/teams/technology/
Hiring: https://careers.duolingo.com
Hiring: https://careers.getyourguide.com/
Hiring: https://careers.gsma.com/current-vacancies/
Hiring: https://careers.jimdo.com/
Hiring: https://careers.li.me
Hiring: https://careers.ovh.com
Hiring: https://careers.rba.gov.au
Hiring: https://careers.ricardo.ch/
Hiring: https://careerssearch.bbc.co.uk/jobs/search
Hiring: https://careers.sharpspring.com/
Hiring: https://careers.technicolor.com/search-jobs/Security
Hiring: https://careers.walmart.com
Hiring: https://career.synology.com/HQ/
Hiring: https://condenastuk.avature.net/careers/SearchJobs
Hiring: https://copyleaks.com/jobs
Hiring: https://corp.inspire.com/meet-inspire/careers/
Hiring: https://corporate.arkadium.com/careers/
Hiring: https://corporate.elisa.com/careers/avoimet_tyopaikat/kaikki_avoimet_paikat/
Hiring: https://corp.roblox.com/careers/
Hiring: https://dat.to/infosecjobs
Hiring: https://db.de/infosec-jobs
Hiring: https://diligent.com/company/career-listings
Hiring: https://doclerholding.recruitee.com/
Hiring: https://duo.com/about/careers
Hiring: https://engoo.com/app/jobs
Hiring: https://evl.ms/obey
Hiring: https://farfetchcareers.com/
Hiring: https://fivetran.com/careers
Hiring: https://fountain.com/careers
Hiring: https://g.co/SecurityPrivacyEngJobs
Hiring: https://goodwillnne.org/jobs/find-a-goodwill-job/
Hiring: https://hcahealthcare.com/careers/search.dot?jobClass=HRPayroll
Hiring: https://heinlein-support.de/jobs
Hiring: https://hevodata.com/careers/
Hiring: https://hopin.com/careers
Hiring: https://hurb.gupy.io/
Hiring: https://inside.lovoo.com/jobs/
Hiring: https://instacart.careers/current-openings/
Hiring: https://iskn.welcomekit.co/
Hiring: https://it.esky.pl/dolacz-do-nas/
Hiring: https://jobba.hemnet.se/jobs
Hiring: https://jobb.tv2.no/digital
Hiring: https://job.ozon.ru/
Hiring: https://jobs.adafruit.com
Hiring: https://jobs.backmarket.com/
Hiring: https://jobsearch.paypal-corp.com/search?keywords=Cyber
Hiring: https://jobs.ebayclassifiedsgroup.com/ebay-kleinanzeigen
Hiring: https://jobs.ebayclassifiedsgroup.com/search-jobs/Denmark
Hiring: https://jobs.kroger.com/search/?q=security&title=technology&department=060
Hiring: https://jobs.mollie.com/
Hiring: https://jobs.vanmoof.com/
Hiring: https://jobs.yoopies.com/
Hiring: https://join.jfrog.com
Hiring: https://join.linehub.com/
Hiring: https://join.tts.gsa.gov/join/security-ops-engineer/
Hiring: https://kariera.shoptet.cz/
Hiring: https://lifeatexpedia.com/
Hiring: https://lightcurve.io/careers
Hiring: https://linecorp.com/ja/career/position/498
Hiring: https://linode.com/careers
Hiring: https://messagebird.com/en/careers/
Hiring: https://monzo.com/careers/
Hiring: https://my.hirehive.io/ninjarmm-llc/jobs/74277/red-team-security-engineer
Hiring: https://n26.com/careers
Hiring: https://namshi.com/careers
Hiring: https://nextdoor.com/jobs
Hiring: https://nubank.com.br/en/careers/
Hiring: https://orange.jobs/
Hiring: https://padlet.jobs
Hiring: https://platform.sh/company/careers/
Hiring: https://preply.com/en/careers
Hiring: https://protonmail.com/careers
Hiring: https://rasa.com/jobs
Hiring: https://recruit.jobcan.jp/pixiv/show/b001/19024
Hiring: https://recrutement.decathlon.fr/
Hiring: https://resources.wellthy.com/careers/
Hiring: https://rewind.io/about/careers/
Hiring: https://schroders.referrals.selectminds.com/careers/
Hiring: https://slite.com/jobs
Hiring: https://snyk.io/jobs/
Hiring: https://specopssoft.com/careers/
Hiring: https://stackoverflow.com/jobs/companies/puregym
Hiring: https://system76.com/careers
Hiring: https://team.ansarada.com/all
Hiring: https://team.lovecrafts.com/
Hiring: https://tech.showmax.com/open-positions/
Hiring: https://ukfast.jobs
Hiring: https://uphold.com/en/about-us/careers
Hiring: https://vercel.com/careers
Hiring: https://vimeo.com/jobs#engineering
Hiring: https://vinted.com/jobs/
Hiring: https://volunteers.joomla.org/teams/security-strike-team
Hiring: https://werkenbij.wehkamp.nl/
Hiring: https://wise.jobs/
Hiring: https://wolt.com/en/jobs
Hiring: https://workat.doximity.com/positions/
Hiring: https://workforus.theguardian.com/index.php/search-jobs-and-apply/?search_paths%5B%5D=&query=information+security&submit=Search+Jobs
Hiring: https://ww2.glance.net/about/careers/
Hiring: https://www.23andme.com/careers/
Hiring: https://www.acronis.com/en-us/careers/
Hiring: https://www.adaware.com/careers
Hiring: https://www.ahgora.com.br/hcm/contato/
Hiring: https://www.amobee.com/company/careers/
Hiring: https://www.ankama.com/en/recruitment
Hiring: https://www.ao-jobs.com
Hiring: https://www.atlassian.com/company/careers/all-jobs?team=Security
Hiring: https://www.axonius.com/company/careers/open-jobs
Hiring: https://www.binance.com/Careers.html
Hiring: https://www.blablacar.com/dreamjobs
Hiring: https://www.bloomandwild.com/careers
Hiring: https://www.bugcrowd.com/about/careers/
Hiring: https://www.canva.com/careers/jobs/?specialty=security
Hiring: https://www.catalyst-au.net/jobs
Hiring: https://www.checkout.com/careers
Hiring: https://www.civilservicejobs.service.gov.uk/
Hiring: https://www.cleverbridge.com/corporate/about-us/careers/
Hiring: https://www.clio.com/about/careers/
Hiring: https://www.cloudflare.com/careers/jobs/
Hiring: https://www.cockroachlabs.com/careers/
Hiring: https://www.codeweavers.com/about/jobs
Hiring: https://www.contentful.com/careers/
Hiring: https://www.creditkarma.com/careers
Hiring: https://www.csfd.cz/vyvojari/
Hiring: https://www.darktrace.com/en/careers/
Hiring: https://www.deepnote.com/work
Hiring: https://www.deere.com/en/our-company/john-deere-careers/
Hiring: https://www.deezerjobs.com
Hiring: https://www.despegar.com/sumate
Hiring: https://www.dreamhost.com/careers/
Hiring: https://www.dropbox.com/jobs/search?q=security
Hiring: https://www.eitb.eus/eu/eitb-taldea/lan-poltsa/
Hiring: https://www.emsisoft.com/en/company/jobs/
Hiring: https://www.epam.com/careers/job-listings/job.24901.security-testing-engineer_minsk_belarus
Hiring: https://www.exabeam.com/careers/
Hiring: https://www.expressvpn.com/jobs
Hiring: https://www.flywire.com/company/careers
Hiring: https://www.fortnox.se/om-fortnox/jobba-pa-fortnox/
Hiring: https://www.genetec.com/careers/life-at-genetec
Hiring: https://www.guilded.gg/jobs
Hiring: https://www.handy.com/careers/
Hiring: https://www.hidglobal.com/careers
Hiring: https://www.html.it/jobs.html
Hiring: https://www.hyperscience.com/careers/#view-openings
Hiring: https://www.jumbointeractive.com/careers
Hiring: https://www.jumpship.co.nz/home
Hiring: https://www.kiwi.com/jobs/
Hiring: https://www.klei.com/careers
Hiring: https://www.kreezalid.com/jobs
Hiring: https://www.lazada.com/en/careers/job-search/?keywords=security
Hiring: https://www.lhv.ee/en/careers
Hiring: https://www.linkedin.com/company/visier-analytics/
Hiring: https://www.linkedin.com/company/zopa
Hiring: https://www.nexus.org/nexus-iba/employment-opportunities/
Hiring: https://www.northwesternmutual.com/careers/
Hiring: https://www.npmjs.com/jobs
Hiring: https://www.nuspire.com/careers/
Hiring: https://www.offensive-security.com/careers/
Hiring: https://www.ookla.com/about/careers
Hiring: https://www.opentext.com/about/careers
Hiring: https://www.otto.de/unternehmen/jobs/
Hiring: https://www.palantir.com/careers/
Hiring: https://www.pandadoc.com/careers/
Hiring: https://www.papercut.com/about/life-at-papercut/
Hiring: https://www.patreon.com/careers
Hiring: https://www.pentestpartners.com/about-us/careers/
Hiring: https://www.playtika.com/careers
Hiring: https://www.pluralsight.com/careers/jobs
Hiring: https://www.productboard.com/careers/open-positions/
Hiring: https://www.puregym.com/careers/
Hiring: https://www.qualtrics.com/careers/
Hiring: https://www.qualys.com/careers/
Hiring: https://www.securities.io/contact-us/
Hiring: https://www.securitymetrics.com/careers
Hiring: https://www.shopify.com/careers
Hiring: https://www.siteminder.com/careers
Hiring: https://www.skillshare.com/careers
Hiring: https://www.slevomat.cz/prace
Hiring: https://www.smule.com/jobs
Hiring: https://www.snappcar.nl/werken-bij
Hiring: https://www.sonarsource.com/company/jobs/
Hiring: https://www.sonarsource.com/hiring
Hiring: https://www.spaceapegames.com/careers
Hiring: https://www.spacex.com/careers/?department=Information%2520Security
Hiring: https://www.spacex.com/careers/index.html?department=Software%2520Development
Hiring: https://www.spring-media.de/en/front-page/#jobs
Hiring: https://www.stonex.com/careers
Hiring: https://www.stormshield.com/join-us/offers/?_sft_offer_type=permanent-contract&_sft_offer_service=rd
Hiring: https://www.svb.com/careers
Hiring: https://www.switch.ch/de/about/jobs/
Hiring: https://www.telegramgrupa.hr/jobs.php
Hiring: https://www.tenable.com/careers
Hiring: https://www.therapynotes.com/careers/
Hiring: https://www.thirdlight.com/about/careers/
Hiring: https://www.tide.co/careers
Hiring: https://www.tinkoff.ru/career/it/?specialtyUrl=informacionnaya-bezopasnost
Hiring: https://www.truework.com/careers
Hiring: https://www.tumblr.com/jobs
Hiring: https://www.usaajobs.com/search-jobs/information%20security/
Hiring: https://www.verizonmedia.com/careers/search.html?q=paranoids
Hiring: https://www.webgains.com/public/en/about/#hiring
Hiring: https://www.welcometothejungle.co/companies/gandi
Hiring: https://www.workiva.com/careers
Hiring: https://www.xing.com/jobs/search?keywords=%22NEW%20WORK%20SE%22%20Security
Hiring: http://www.uu.se/jobb/
Hiring: jobs@bidorbuy.co.za
Hiring: kariera@ceneo.pl
Hiring: mailto:security@sparebank1.no
Hiring: security@percona.com
Hiring: <span title="" class="pep-email">hr(Replace this parenthesis with the @ sign)ajsmart.com</span><br>	
```

</details>