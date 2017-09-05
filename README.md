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
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 2, nanos: 501921052 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9504 }, throughput (mb/sec): 417.3786
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6317136, elapsed: Duration { secs: 4, nanos: 45194591 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15367 }, throughput (mb/sec): 258.1454
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 1, nanos: 329386600 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5050 }, throughput (mb/sec): 785.5113
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 263214, size: 1094973642, r: 6843564, elapsed: Duration { secs: 0, nanos: 930954699 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3536 }, throughput (mb/sec): 1121.6961

number of queries = 2
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8144696, elapsed: Duration { secs: 2, nanos: 532808417 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 9622 }, throughput (mb/sec): 412.2887
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 7618268, elapsed: Duration { secs: 4, nanos: 116380076 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15638 }, throughput (mb/sec): 253.6812
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 1, nanos: 403057993 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5330 }, throughput (mb/sec): 744.2659
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 263214, size: 1094973642, r: 8206912, elapsed: Duration { secs: 0, nanos: 972142706 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3693 }, throughput (mb/sec): 1074.1718

number of queries = 4
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 33848920, elapsed: Duration { secs: 2, nanos: 994489581 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 11376 }, throughput (mb/sec): 348.7233
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 32796064, elapsed: Duration { secs: 4, nanos: 198880939 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15951 }, throughput (mb/sec): 248.6968
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 406501256 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5343 }, throughput (mb/sec): 742.4439
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 263214, size: 1094973642, r: 35287798, elapsed: Duration { secs: 1, nanos: 27777801 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3904 }, throughput (mb/sec): 1016.0253

number of queries = 8
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 110592552, elapsed: Duration { secs: 4, nanos: 100129087 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15576 }, throughput (mb/sec): 254.6867
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 108486840, elapsed: Duration { secs: 4, nanos: 524139997 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 17187 }, throughput (mb/sec): 230.8170
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 483059190 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5634 }, throughput (mb/sec): 704.1177
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 263214, size: 1094973642, r: 117254690, elapsed: Duration { secs: 1, nanos: 129571110 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4291 }, throughput (mb/sec): 924.4644

number of queries = 16
file_path: work/companies_1g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 144763472, elapsed: Duration { secs: 4, nanos: 855081200 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18444 }, throughput (mb/sec): 215.0836
file_path: work/companies_1g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 139591312, elapsed: Duration { secs: 4, nanos: 767484698 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18111 }, throughput (mb/sec): 219.0355
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 612063202 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6124 }, throughput (mb/sec): 647.7713
file_path: work/companies_1g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 263214, size: 1094973642, r: 151425610, elapsed: Duration { secs: 1, nanos: 408694131 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5351 }, throughput (mb/sec): 741.2881
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
