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
    Finished release [optimized] target(s) in 0.0 secs

number of queries = 1
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 2, nanos: 487382750 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9449 }, throughput (mb/sec): 419.8181
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6317136, elapsed: Duration { secs: 4, nanos: 190898966 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15921 }, throughput (mb/sec): 249.1705
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 313350620 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4989 }, throughput (mb/sec): 795.1024
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 0, nanos: 903027576 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3430 }, throughput (mb/sec): 1156.3858

number of queries = 2
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8144696, elapsed: Duration { secs: 2, nanos: 585043423 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9820 }, throughput (mb/sec): 403.9577
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 4, nanos: 246430355 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16132 }, throughput (mb/sec): 245.9120
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 348329119 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5122 }, throughput (mb/sec): 774.4758
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 0, nanos: 944867480 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3589 }, throughput (mb/sec): 1105.1796

number of queries = 4
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 33848920, elapsed: Duration { secs: 3, nanos: 102237124 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 11785 }, throughput (mb/sec): 336.6114
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 4, nanos: 262629995 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16193 }, throughput (mb/sec): 244.9775
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 359750708 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5165 }, throughput (mb/sec): 767.9704
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 0, nanos: 954944285 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3628 }, throughput (mb/sec): 1093.5175

number of queries = 8
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 110592552, elapsed: Duration { secs: 4, nanos: 155873814 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15788 }, throughput (mb/sec): 251.2704
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 108486840, elapsed: Duration { secs: 4, nanos: 603804594 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 17489 }, throughput (mb/sec): 226.8229
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 422790170 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5405 }, throughput (mb/sec): 733.9440
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 41323593 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3955 }, throughput (mb/sec): 1002.8086

number of queries = 16
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 144763472, elapsed: Duration { secs: 4, nanos: 936942649 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18755 }, throughput (mb/sec): 211.5172
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 139591312, elapsed: Duration { secs: 5, nanos: 396408453 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 20501 }, throughput (mb/sec): 193.5080
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 481092995 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5626 }, throughput (mb/sec): 705.0524
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 278426508 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4856 }, throughput (mb/sec): 816.8231
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
