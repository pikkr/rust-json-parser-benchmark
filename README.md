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
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 2, nanos: 358828673 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 8961 }, throughput (mb/sec): 442.6978
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6317136, elapsed: Duration { secs: 4, nanos: 119015283 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15648 }, throughput (mb/sec): 253.5189
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 240794918 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4713 }, throughput (mb/sec): 841.5962
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 0, nanos: 843788316 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3205 }, throughput (mb/sec): 1237.5713

number of queries = 2
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8144696, elapsed: Duration { secs: 2, nanos: 445069132 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9288 }, throughput (mb/sec): 427.0833
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 3, nanos: 982658287 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15130 }, throughput (mb/sec): 262.1988
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 244289892 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4727 }, throughput (mb/sec): 839.2323
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 0, nanos: 881247467 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3348 }, throughput (mb/sec): 1184.9659

number of queries = 4
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 33848920, elapsed: Duration { secs: 2, nanos: 838800050 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 10784 }, throughput (mb/sec): 367.8485
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 32796064, elapsed: Duration { secs: 4, nanos: 8964630 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15230 }, throughput (mb/sec): 260.4783
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 253067406 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4760 }, throughput (mb/sec): 833.3536
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 0, nanos: 928839538 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3528 }, throughput (mb/sec): 1124.2504

number of queries = 8
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 110592552, elapsed: Duration { secs: 3, nanos: 883976598 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14755 }, throughput (mb/sec): 268.8606
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 108486840, elapsed: Duration { secs: 5, nanos: 308246961 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 20166 }, throughput (mb/sec): 196.7219
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 337211943 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5080 }, throughput (mb/sec): 780.9145
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 16976823 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3863 }, throughput (mb/sec): 1026.8162

number of queries = 16
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 144763472, elapsed: Duration { secs: 4, nanos: 599155280 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 17472 }, throughput (mb/sec): 227.0522
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 139591312, elapsed: Duration { secs: 4, nanos: 835917924 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18371 }, throughput (mb/sec): 215.9359
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 418699854 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5389 }, throughput (mb/sec): 736.0600
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 230036514 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4672 }, throughput (mb/sec): 848.9571
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
