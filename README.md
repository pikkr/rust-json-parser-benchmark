# Rust JSON Parser Benchmark

## Download and Generate JSON Data

```
$ mkdir work
$ cd work
$ curl -O http://jsonstudio.com/wp-content/uploads/2014/02/companies.zip
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 14.7M  100 14.7M    0     0  2610k      0  0:00:05  0:00:05 --:--:-- 3428k

$ unzip companies.zip
Archive:  companies.zip
  inflating: companies.json

$ for i in {1..14}; do cat companies.json >> companies_1g.json; done
$ for i in {1..2}; do cat companies_1g.json >> companies_2g.json; done
$ for i in {1..2}; do cat companies_2g.json >> companies_4g.json; done
$ for i in {1..2}; do cat companies_4g.json >> companies_8g.json; done
```

## Run Benchmark

```
$ ./bench.sh
build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading pikkr v0.14.0
   Compiling pikkr v0.14.0
   Compiling rjpb v0.1.0 (file:///Users/JP21605/dev/rust-json-parser-benchmark)
    Finished release [optimized] target(s) in 6.96 secs

number of queries = 1
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 2, nanos: 315596851 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 8797 }, throughput (mb/sec): 450.9629
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6317136, elapsed: Duration { secs: 3, nanos: 847926423 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14618 }, throughput (mb/sec): 271.3795
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 171662607 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4451 }, throughput (mb/sec): 891.2534
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 0, nanos: 805589673 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3060 }, throughput (mb/sec): 1296.2533

number of queries = 2
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8144696, elapsed: Duration { secs: 2, nanos: 374926437 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9022 }, throughput (mb/sec): 439.6971
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 3, nanos: 947734176 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14997 }, throughput (mb/sec): 264.5184
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 187955851 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4513 }, throughput (mb/sec): 879.0295
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 0, nanos: 828500252 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3147 }, throughput (mb/sec): 1260.4079

number of queries = 4
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 33848920, elapsed: Duration { secs: 2, nanos: 795513908 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 10620 }, throughput (mb/sec): 373.5443
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 32796064, elapsed: Duration { secs: 4, nanos: 63429747 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15436 }, throughput (mb/sec): 256.9869
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 185588629 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4504 }, throughput (mb/sec): 880.7846
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 0, nanos: 878498212 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3337 }, throughput (mb/sec): 1188.6743

number of queries = 8
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 110592552, elapsed: Duration { secs: 3, nanos: 804711745 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14454 }, throughput (mb/sec): 274.4619
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 108486840, elapsed: Duration { secs: 4, nanos: 416617711 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16778 }, throughput (mb/sec): 236.4362
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 283099280 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4874 }, throughput (mb/sec): 813.8484
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 0, nanos: 955779928 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3631 }, throughput (mb/sec): 1092.5614

number of queries = 16
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 144763472, elapsed: Duration { secs: 4, nanos: 457018407 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16932 }, throughput (mb/sec): 234.2930
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 139591312, elapsed: Duration { secs: 4, nanos: 766355832 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18107 }, throughput (mb/sec): 219.0873
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 335044589 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5071 }, throughput (mb/sec): 782.1823
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 160745581 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4409 }, throughput (mb/sec): 899.6358
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
