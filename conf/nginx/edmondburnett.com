proxy_cache_path /var/cache/nginx-edmondburnett-com levels=1:2 keys_zone=edmondburnett_com_cache:10m max_size=1g inactive=1h use_temp_path=off;

server {
    listen 80;
    listen [::]:80;
    server_name edmondburnett.com www.edmondburnett.com;
    server_tokens off;
    return 301 https://edmondburnett.com$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name edmondburnett.com www.edmondburnett.com;

    server_tokens off;
    ssl_certificate /etc/letsencrypt/live/edmondburnett.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/edmondburnett.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    add_header Content-Security-Policy "default-src 'none'; form-action 'self'; base-uri 'self'; frame-ancestors 'self'; script-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self'; style-src 'self'; frame-src 'self' *.youtube.com *.spotify.com *.soundcloud.com; font-src 'self' *.googleapis.com *.gstatic.com;" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
    add_header X-Content-Type-Options nosniff;
    add_header X-Frame-Options SAMEORIGIN;

    access_log /var/log/nginx/edmondburnett.com.access.log;
    error_log /var/log/nginx/edmondburnett.com.error.log;

    if ($host = www.edmondburnett.com) {
        return 301 https://edmondburnett.com$request_uri;
    }

    location /static/ {
        alias /srv/http/edmondburnett.com/static/;
        access_log off;
    }

    location / {
        proxy_pass http://127.0.0.1:7000;

        proxy_cache edmondburnett_com_cache;
        proxy_cache_valid 200 10m;
        proxy_cache_key "$scheme$request_method$host$request_uri";
        proxy_cache_use_stale error timeout updating http_500 http_502 http_503 http_504;

        proxy_http_version 1.1;
        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # block unwanted user agents
    if ($http_user_agent ~* (LWP::Simple|wget|libwww-perl|360Spider|80legs|Abonti|AcoonBot|Acunetix|adbeat_bot|AddThis|adidxbot|ADmantX|AhrefsBot|AngloINFO|Antelope|Applebot|BaiduSpider|BeetleBot|billigerbot|binlar|bitlybot|BlackWidow|BLP_bbot|BoardReader|Bolt|BOT.*JCE|casper|CazoodleBot|CCBot|checkprivacy|ChinaClaw|chromeframe|Clerkbot|Cliqzbot|clshttp|CommonCrawler|comodo|CPython|crawler4j|Crawlera|CRAZYWEBCRAWLER|Curious|Curl|Custo|CWS_proxy|diavol|DigExt|Digincore|DIIbot|discobot|DISCo|DoCoMo|DotBot|Download.*Demon|DTS\.Agent|EasouSpider|eCatch|ecxi|EirGrabber|Elmer|EmailCollector|EmailSiphon|EmailWolf|Exabot|ExaleadCloudView|ExpertSearchSpider|Extract|EyeNetIE|Ezooms|F2S|FastSeek|feedfinder|FeedlyBot|FHscan|finbot|Flamingo|FlappyBot|FlashGet|flicky|Flipboard|g00g1e|Genieo|GetRight|GetWeb|GigablastOpenSource|GozaikBot|GrabNet|Grafula|GrapeshotCrawler|GTB5|Guzzle|harvest|heritrix|HMView|HomePageBot|HTTrack|HubSpot|ia_archiver|icarus6|IDBot|id-search|IlseBot|Image.*Stripper|Image.*Sucker|Indigonet|Indy.*Library|integromedb|InterGET|InternetSeer|Internet.*Ninja|IRLbot|jakarta|Java|JetCar|JobdiggerSpider|JOC.*Spider|Jooblebot|kanagawa|KINGSpider|kmccrew|larbin|LeechFTP|libwww|Lingewoud|LinkChecker|linkdexbot|LinksCrawler|LinksManager|linkwalker|LinqiaRSSBot|LivelapBot|ltx71|LubbersBot|lwp|Mail\.RU|masscan|Mass.*Downloader|maverick|Maxthon|Mediatoolkitbot|MegaIndex|MFC_Tear|Microsoft.*URL|miner|Missigua|Mister.*PiX|mj12bot|MSFrontPage|msnbot|Navroad|NearSite|NetAnts|netEstate|NetSpider|NetZIP|Net.*Vampire|NextGenSearchBot|nutch|Octopus|Offline.*Explorer|Offline.*Navigator|OpenindexSpider|OpenWebSpider|OrangeBot|Owlin|PageGrabber|PagesInventory|panopta|panscient|Papa.*Foto|pavuk|pcBrowser|PECL|PeoplePal|Photon|PHPCrawl|planetwork|PleaseCrawl|PNAMAIN|PodcastPartyBot|prijsbest|proximic|psbot|purebot|pycurl|QuerySeekerSpider|R6_|RealDownload|ReGet|Riddler|Rippers|rogerbot|RSSingBot|RyzeCrawler|SafeSearch|SBIder|Scrapy|Screaming|SeaMonkey|search\.goo|SearchmetricsBot|search_robot|SemrushBot|Semrush|SentiBot|SEOkicks|SeznamBot|ShowyouBot|SightupBot|SISTRIX|sitecheck|siteexplorer|SiteSnagger|skygrid|Slackbot|Slurp|SmartDownload|Snoopy|Sogou|Sosospider|spaumbot|Steeler|sucker|SuperBot|Superfeedr|SuperHTTP|SurdotlyBot|Surfbot|tAkeOut|Teleport|TinEye|Toata.*diavola|Toplistbot|trendictionbot|TurnitinBot|URI.*Fetch|urllib|Vagabondo|vikspider|VoidEYE|VoilaBot|WBSearchBot|webalta|WebAuto|WebBandit|WebCollage|WebCopier|WebFetch|WebGo|WebLeacher|WebReaper|WebSauger|Website.*eXtractor|Website.*Quester|WebStripper|WebWhacker|WebZIP|Web.*Image.*Collector|Web.*Sucker|Wells.*Search|WEP.*Search|WESee|Widow|WinInet|woobot|woopingbot|worldwebheritage|Wotbox|WPScan|WWWOFFLE|WWW-Mechanize|Xaldon|XoviBot|yacybot|YandexBot|Yandex|YisouSpider|zermelo|Zeus|ZmEu|ZumBot|ZyBorg|meta-externalagent)) {
        return 403;
    }
}
