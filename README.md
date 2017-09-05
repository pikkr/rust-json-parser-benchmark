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
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 2, nanos: 416370890 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9179 }, throughput (mb/sec): 432.1556
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6317136, elapsed: Duration { secs: 4, nanos: 71129015 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15466 }, throughput (mb/sec): 256.5009
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 439338924 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5468 }, throughput (mb/sec): 725.5055
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 20452175 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3876 }, throughput (mb/sec): 1023.3191

number of queries = 2
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8144696, elapsed: Duration { secs: 2, nanos: 531845929 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9618 }, throughput (mb/sec): 412.4454
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 4, nanos: 82502824 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15509 }, throughput (mb/sec): 255.7863
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 445556781 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5491 }, throughput (mb/sec): 722.3848
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 73441702 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4078 }, throughput (mb/sec): 972.8039

number of queries = 4
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 33848920, elapsed: Duration { secs: 3, nanos: 3713406 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 11411 }, throughput (mb/sec): 347.6524
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 32796064, elapsed: Duration { secs: 4, nanos: 230153929 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16070 }, throughput (mb/sec): 246.8582
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 493554378 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5674 }, throughput (mb/sec): 699.1699
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 110824384 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4220 }, throughput (mb/sec): 940.0660

number of queries = 8
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 110592552, elapsed: Duration { secs: 4, nanos: 35360125 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15330 }, throughput (mb/sec): 258.7745
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 108486840, elapsed: Duration { secs: 4, nanos: 484063324 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 17035 }, throughput (mb/sec): 232.8799
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 588404631 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6034 }, throughput (mb/sec): 657.4195
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 202862597 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4569 }, throughput (mb/sec): 868.1359

number of queries = 16
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 144763472, elapsed: Duration { secs: 4, nanos: 778029512 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18151 }, throughput (mb/sec): 218.5521
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 139591312, elapsed: Duration { secs: 4, nanos: 850782172 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18428 }, throughput (mb/sec): 215.2742
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 735977674 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6595 }, throughput (mb/sec): 601.5332
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 477155628 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5611 }, throughput (mb/sec): 706.9318
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
