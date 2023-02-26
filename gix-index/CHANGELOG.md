# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.12.4 (2023-02-20)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
</details>

## 0.12.3 (2023-02-17)

<csr-id-dc3fd48327a16bf8599eaf70b55ab6c1d754e4bb/>
<csr-id-4a6d46f3ab3d15eb851c92f7e49eb6772bc4023b/>
<csr-id-6c17f96fcee9e2935b464c8ffbd30b253d9f5a6c/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### New Features

 - <csr-id-5dc408f726d6f0f480438348eb5d713776329710/> read shared indices by dissolving them into the current one.
   This allows the 'link' extension to be processed correctly, even though it
   won't be maintained. When written back, the 'link' extension will be removed
   automatically.
 - <csr-id-6d8eb9f267ac7ef8db0f3a277c8c991df5ce8164/> add `impl Debug for State` to print all entries in a digestible form.
 - <csr-id-7c8ba2c945d6313d27569f04b83ebf9a2387e6a2/> add `State::sort_entries_by(...)` to allow comparing by additional criteria.
   This allows to orgranize entries based on flags, for example, which may help
   in special occasions.
 - <csr-id-1e3341a77c089e6d80c271fca46b774b01b01386/> `entry::Time` can convert from and to system time.
 - <csr-id-654bd8f62d5546b0f57e82d1be8211431685c7ce/> add `State::sort_entries()` and `State::dangerously_push_entry()`.
   Both methods work in tandem as one breaks invariants, but allows to quickly
   add entries, while the other restores them.
 - <csr-id-aa1b6ee1edeaebae1237184c635f69fdbb23ffee/> add `State::entry_mut_by_path_and_stage()`
 - <csr-id-3ebe2d490b2dbdcb6fe0115b06f205f1c4008fff/> `State::write_to()` respects the `REMOVED` flag.
   That way, one can mark entries for removal and these will be pruned
   at write time. This is preferable over performing removals expensively
   in memory.
 - <csr-id-ec365866c746247b7d5d47b88d51d9cc255724fb/> expose `git_hash` as `hash` in the root of the crate.
   This makes it easier to use the crate standalone as plumbing as instantiation
   requires access to `git_hash`.
 - <csr-id-5cc3a15a8085a67701fc1f2d3ba0e55f71d0a4c0/> add `File::at_or_default(...)` to easily open or create an empty in-memory index.
   This is a common operation in new repositories.
 - <csr-id-0b40951f0fb9ee354a83751264ed89d0608111f8/> add `State::new()` to create a new empty in-memory index.
 - <csr-id-a7183e28513fa1e1a1a784b759677f0e4b4db5f4/> add `File::set_path()` to allow setting the location of an index file on disk.
   This is useful to change the location of an index after reading it (from another
   location, similar to copy-on-write).
 - <csr-id-fd2dd3a74c8d64407c1c27f29a2914389ded3bd6/> name spawned threads
   That way it's a bit more obvious what's happening when the CPU goes
   up in flames.
 - <csr-id-458e1bcbd7043f0759f7445bfa46189910baff54/> Clnoe for `File`
 - <csr-id-9e03110578bd93da3f1a91c5bcd9fde942c81ac4/> `decode::Options::default_from_object_hash()`
   An easier way to initialize decode options, providing only the mandatory
   information.
 - <csr-id-eedcffa728c7e895da51d5298db28f3fef05f7da/> `File::write()` for secure and complete writing of index files.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-6d8d5e6198dfb4d648061807ed4f96868a36ee52/> `Stage::entry_index_by_path_and_stage()`, now with `::entry_by_path_and_stage()`
 - <csr-id-55363ea650001b7717545b4d2968419707a3b8c6/> `State::entry_by_path_and_stage()` to find entries.
 - <csr-id-40e6bde125778e3b50999331c4ed5a4b119937fa/> `Debug` and `Clone` for `File`
 - <csr-id-8ab219ac47ca67f2478b8715d7820fd6171c0db2/> `State::path_backing()`.
   That way it's possible to call certain methods that take a separate
   path buffer.
 - <csr-id-645ed50dc2ae5ded2df0c09daf4fe366b90cf47e/> support for separating lifetimes of entries and path-backing
   This way it should be possible to access paths immutably even while
   entries are available mutably, assuming we stagger accesses to put
   mutation of entries last.

### Refactor

 - <csr-id-dc3fd48327a16bf8599eaf70b55ab6c1d754e4bb/> inline bounds checks
   With `FnMut` one can pass any data to mutable state available via references
   and thus bypass limitations due to the closure's return type.
   
   The `gix-bitmap` crate was kept simple to encourage using it this way.

### Reverted (BREAKING)

 - <csr-id-bd312acf5ceba28edf2508aef6011c037eb0a377/> `decode::Options::default()` - remove `Default` impl.
   The contained `git_hash::Kind` can't actually be defaulted as we
   have to know the actual kind used in the repository.
 - <csr-id-2da5a62432350ede6b816254c894863d14aa4ba1/> remove `write::Options::default()`.
   In practice it's required to inform about the hash kind to use and it's
   possibly incorrect to assume Sha1.

### Bug Fixes (BREAKING)

 - <csr-id-0b1543d481337ed51dcfdeb907af21f0bc98dcb9/> lower rust edition to 2018

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Changed (BREAKING)

 - <csr-id-3753ad58a8303fbf325b07757b2b97c34253bca4/> remove `File::into_state()` in favor of `From<File> for State`.
   That way it's less discoverable, but more idiomatic, and we don't want to
   get into the habit of providing multiple names of the exact same
   functionality.
 - <csr-id-59f679126aba6f8a432aeb53f0bbd5d136ec1deb/> `write::Options::object_hash` is now implied by the `State` itself.
   The `State`, once initialized, knows the kind of object hash it uses and
   there is no need to specify it again.
   
   This affects some method signatures which now work without
   `object_hash`.
 - <csr-id-908163ab2f86a1b603e69f04cd857fbf52e5abfb/> `decode::Options::object_hash` is now a parameter to methods.
   It's not actually an option that could be defaulted, but an integral
   piece of knowledge that must always be defined by the caller.
   
   This also makes `decode::Options::default()` available once again.
 - <csr-id-92dda50e2d9c584b0e110026f59fb715ec41600a/> seal `File` members to preserve consistency better.
   This also makes sure that it's obvious if the `checksum` is actually
   already computed.
 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Other

 - <csr-id-4a6d46f3ab3d15eb851c92f7e49eb6772bc4023b/> sketch out how a write implementation could work
 - <csr-id-6c17f96fcee9e2935b464c8ffbd30b253d9f5a6c/> :init module

### Bug Fixes

 - <csr-id-c2cc939d131a278c177c5f44d3b26127c65bd352/> lower MSRV to 1.52

### Chore

 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 433 commits contributed to the release over the course of 903 calendar days.
 - 37 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 11 unique issues were worked on: [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#333](https://github.com/Byron/gitoxide/issues/333), [#384](https://github.com/Byron/gitoxide/issues/384), [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Remove performance bottleneck when reading TREE extension ([`50411c8`](https://github.com/Byron/gitoxide/commit/50411c8031e3103bb84da46b94b8faf92c597df9))
    - Assert store tree cache matches actual source objects ([`b062bec`](https://github.com/Byron/gitoxide/commit/b062becd01058f5c519538f89d9d8fec8342114d))
    - Sketch a surprisingly difficult way of loading objects in verify_extension() ([`3baeab4`](https://github.com/Byron/gitoxide/commit/3baeab4ab216132536d5c182b3e316ce65095085))
    - Properly sort cache tree entries upon load ([`421d1ba`](https://github.com/Byron/gitoxide/commit/421d1ba853a75560f59cb0ce2b353087db0dff56))
    - tree-ordering validation shows something wrong ([`5fb2857`](https://github.com/Byron/gitoxide/commit/5fb2857e9f970c150f5221ca721506e7bc046ef4))
    - First stab at tree verification ([`f928350`](https://github.com/Byron/gitoxide/commit/f9283500e8316ab949fc0ff9c2fc13a498380873))
    - Verify entry order ([`2d101eb`](https://github.com/Byron/gitoxide/commit/2d101ebbd36e000ffec0e965012081fec2e234f7))
    - refactor ([`017e915`](https://github.com/Byron/gitoxide/commit/017e9153aaaa1cdd6788d9f61ff1ffbd61bc1b30))
    - basic index file checksum verification ([`c644565`](https://github.com/Byron/gitoxide/commit/c644565d5b8d9ae3991ee82a7ffa5e21a2705f91))
    - At least check for the presence of extensions ([`28c056c`](https://github.com/Byron/gitoxide/commit/28c056c6d2bbfb16a826238fd6879adecbeb1171))
    - thorough checking of Tree extension ([`d1063aa`](https://github.com/Byron/gitoxide/commit/d1063aa20bfcefb064ba08089f095baef1299dcd))
    - refactor ([`d0725bd`](https://github.com/Byron/gitoxide/commit/d0725bd40f0b9af0e0af34ffe77c2d8406c6d24c))
    - Fix tree-extension loading for empty trees ([`2e13989`](https://github.com/Byron/gitoxide/commit/2e1398985edfaf9e62ff5643cf4756d9d9717862))
    - Now we are able to load indices correctly ([`762efa3`](https://github.com/Byron/gitoxide/commit/762efa3f5e4ebda4d3bcc6a9bba43c6cdb407937))
    - Add breaking test with conflicting file in index ([`791a9f8`](https://github.com/Byron/gitoxide/commit/791a9f84ff8871c7beb0e2100a4dcba0e9384737))
    - Print extension names instead of count ([`1cc07e0`](https://github.com/Byron/gitoxide/commit/1cc07e0cfdae74e388abb29d7acb9c6f643278b4))
    - Print basic index information, including the tree extension ([`9277cf8`](https://github.com/Byron/gitoxide/commit/9277cf877e1f2276dcad1efdeb97e0e3d96ed3f0))
    - lower rust edition to 2018 ([`0b1543d`](https://github.com/Byron/gitoxide/commit/0b1543d481337ed51dcfdeb907af21f0bc98dcb9))
    - lower MSRV to 1.52 ([`c2cc939`](https://github.com/Byron/gitoxide/commit/c2cc939d131a278c177c5f44d3b26127c65bd352))
    - prepare changelogs for git-index and dependencies ([`f54bf4b`](https://github.com/Byron/gitoxide/commit/f54bf4bde92b892b6d425987a6a37e10319c4635))
    - Test for extra-long paths ([`3d61afe`](https://github.com/Byron/gitoxide/commit/3d61afe615227e2af96525eba5b0e8e2f94207f3))
    - Test for extended flags ([`ae3b697`](https://github.com/Byron/gitoxide/commit/ae3b69710cf316cb8164120d4b98f051eef363bc))
    - Use bitflags for Flags (in-memory and at-rest) ([`ea86eb0`](https://github.com/Byron/gitoxide/commit/ea86eb0bebb0f067fc8710779c2c296632451c54))
    - Use bitflags for entry Mode ([`53df605`](https://github.com/Byron/gitoxide/commit/53df6057a8c50007716d89e08f1efe70435f8613))
    - FSMN V2 decoding ([`04279bf`](https://github.com/Byron/gitoxide/commit/04279bffc8bd43abe85f559634436be789782829))
    - Failing test for fs-monitor V1 ([`625b89a`](https://github.com/Byron/gitoxide/commit/625b89a7786fe9de29c9ad2ca41a734174f53128))
    - Validate UNTR with exclude-file oids ([`20ebb81`](https://github.com/Byron/gitoxide/commit/20ebb81c9ece6c2d693edd6243eaa730fa7cf44c))
    - read remaining pieces of UNTR ([`9d9cc95`](https://github.com/Byron/gitoxide/commit/9d9cc95a24d86cf5f66f1746c09ece032640a892))
    - Make stat parsing more general/reusable ([`c41b933`](https://github.com/Byron/gitoxide/commit/c41b933d14f2e4538928e4fbd682e1017702e69c))
    - refactor ([`a1dc8de`](https://github.com/Byron/gitoxide/commit/a1dc8dedc5d9e1712295131d2332c21f3df4ac45))
    - simplify UNTR directory indexing ([`7857d08`](https://github.com/Byron/gitoxide/commit/7857d08a25eac1c7d4a91f04eb80a83ec5677d1b))
    - flatten UNTR directory list for later access via bitmaps ([`2e39184`](https://github.com/Byron/gitoxide/commit/2e391841af52f88b7a0472179e5dda89cc6c9808))
    - read UNTR directory blocks and bitmaps ([`59f46fe`](https://github.com/Byron/gitoxide/commit/59f46fe134e8619f501c79da4290cadd5548751c))
    - First portion of reading the untracked cache ([`ed2fe5d`](https://github.com/Byron/gitoxide/commit/ed2fe5dbfbcf79ffdcdceed90f6321792cff076d))
    - failing test for UNTR extension ([`223f2cc`](https://github.com/Byron/gitoxide/commit/223f2cc1c88f76dc75ca6706f1f61514ab52e496))
    - Add UNTR extension fixture ([`3c7ba24`](https://github.com/Byron/gitoxide/commit/3c7ba247a3fdab114d9d549de50d6143c7fcce0a))
    - REUC reading works ([`29c1af9`](https://github.com/Byron/gitoxide/commit/29c1af9b2d7b9879a806fc84cfc89ed6c0d7f083))
    - frame and test for REUC exstension ([`229cabe`](https://github.com/Byron/gitoxide/commit/229cabe8de9e1bd244d56d24327b05e3d80dfb6e))
    - add git index with REUC exstension ([`8359fdb`](https://github.com/Byron/gitoxide/commit/8359fdb6c49b263bc7ac2f3105254b83eac47638))
    - Support for 'sdir' extension ([`a38c3b8`](https://github.com/Byron/gitoxide/commit/a38c3b889cfbf1447c87d489d3eb9902e757aa4b))
    - Turn git-bitmap Array into Vec, as it will be able to adjust its size ([`9e99e01`](https://github.com/Byron/gitoxide/commit/9e99e016c17f0d5bcd2ab645261dfac58cb48be5))
    - first stab at decoding ewah bitmaps ([`353a53c`](https://github.com/Byron/gitoxide/commit/353a53ccab5af990e7c384b74b38e5429417d449))
    - 'link' extension decoding to the point where bitmaps are needed ([`e18a2fd`](https://github.com/Byron/gitoxide/commit/e18a2fd68e1c7c06fe2ff9cd1634704313822b5c))
    - support for errors in extensions ([`8971991`](https://github.com/Byron/gitoxide/commit/8971991808b1497b9584b52270259d3c625fa970))
    - failing test for link decoding ([`e1daf18`](https://github.com/Byron/gitoxide/commit/e1daf18041862570fdfe53ff88d879d0c3cb7182))
    - don't forget to fail on unknown mandatory extension ([`f7e2bdd`](https://github.com/Byron/gitoxide/commit/f7e2bdd7dcb21600d2aca7d29d131eefe43f0f4f))
    - Aggregation for index entries loaded in parallel ([`995994a`](https://github.com/Byron/gitoxide/commit/995994a895a6faa4537ae1a6564edc005be96a1a))
    - parallel loading of entries right before reducing them ([`de84a3a`](https://github.com/Byron/gitoxide/commit/de84a3a03bcc9dc3ff71810e35c869f9b73dd38f))
    - Frame for using the new 'scoped threads' feature in git-features ([`6fea17d`](https://github.com/Byron/gitoxide/commit/6fea17d1306679d0454d01aa59adf12cd83c7973))
    - single and multi-threaded index tests ([`a22cb0f`](https://github.com/Byron/gitoxide/commit/a22cb0f1ead9a2f32e43eb2fb378281e592a4ed3))
    - prepare decode options for better control of threads ([`30de988`](https://github.com/Byron/gitoxide/commit/30de988f6a97177fcb32ffce37f4c80f46306a20))
    - cleanup ([`99d7224`](https://github.com/Byron/gitoxide/commit/99d7224baa04c199a7eb7aa2675b39657b0aef6a))
    - Basic IEOT parsing ([`35bdee4`](https://github.com/Byron/gitoxide/commit/35bdee4bf77787bcbe6c3dd715a677e2e46a8ad1))
    - refactor ([`6f04f8b`](https://github.com/Byron/gitoxide/commit/6f04f8b8276de9c6b649642fb7c95eb5ffad77e4))
    - parse V4 delta-paths ([`06640e3`](https://github.com/Byron/gitoxide/commit/06640e3f98f25e9502db7ac68e1967d9fd25e8b2))
    - more thorough tests for more complex repo with more entries ([`273853f`](https://github.com/Byron/gitoxide/commit/273853f1614a0106c60d3d73c3bf72fb57b405e8))
    - The first test to validate an entry ([`f865ef6`](https://github.com/Byron/gitoxide/commit/f865ef6c626c9db39a09416333b6465fdd12c734))
    - Now with counting of consumed bytes in extensions ([`77a062c`](https://github.com/Byron/gitoxide/commit/77a062cdaff1bdf80556301f1e1aa41002af9cef))
    - Use correct post-header slice when parsing entries ([`da556b0`](https://github.com/Byron/gitoxide/commit/da556b0a64ac9ca8eaee62cab163789b55903b3d))
    - All code needed to load extensions… ([`0a03f19`](https://github.com/Byron/gitoxide/commit/0a03f196b7ec4dc1e0e2377c729467781c9e6c2c))
    - a step towards pasing V2 paths ([`01036ad`](https://github.com/Byron/gitoxide/commit/01036ad1bafb6a830734a9dd4f4e2949b8981a30))
    - Most of the entry decoding, name is still missing ([`53e2d75`](https://github.com/Byron/gitoxide/commit/53e2d754262d9752d3b106f7991543986ad5426f))
    - Extensions are optional, and so is their iteration ([`620d2e6`](https://github.com/Byron/gitoxide/commit/620d2e6bd4ef6d3281c096aaf344669bcf49e723))
    - Prepare a more complex test for tree parsing, requires entry parsing ([`e7e0679`](https://github.com/Byron/gitoxide/commit/e7e067977ef440cf3edb8812c0d614b5d8213b58))
    - parse TREE chunk ([`a2ea498`](https://github.com/Byron/gitoxide/commit/a2ea49841a333c8af18fd258781a649214a0ae0b))
    - Get closer to implementing a simple TREE extension decoding ([`49fcb6f`](https://github.com/Byron/gitoxide/commit/49fcb6f6ae9d6ed47e7c0c3ea2aa644d4e8cd264))
    - refactor ([`07e8fb2`](https://github.com/Byron/gitoxide/commit/07e8fb2cb6b7819eb34676ede57808b845298674))
    - the first actual assetion ([`c17240d`](https://github.com/Byron/gitoxide/commit/c17240d0cbd6134a77a69359611789f4eebc727d))
    - refactor ([`d4b3a07`](https://github.com/Byron/gitoxide/commit/d4b3a07489703fb6d5e9b9fb9328741172826db9))
    - refactor ([`9fdd34b`](https://github.com/Byron/gitoxide/commit/9fdd34b634f4f15eb6cf5c2e7912bdc32dd61de6))
    - Fix counting issue, checksum matches now ([`cc33752`](https://github.com/Byron/gitoxide/commit/cc337526365a04a23571123531f1ae565d386bcf))
    - Another big step, even though EOIE checksum is still bugged ([`9ffd523`](https://github.com/Byron/gitoxide/commit/9ffd5231c582a3870c6d25ea870c005e77e32276))
    - right before implementing a traversal over extension chunks ([`79ca582`](https://github.com/Byron/gitoxide/commit/79ca582045dd03434737c779b84c991acf1b0823))
    - refactor ([`9b28b18`](https://github.com/Byron/gitoxide/commit/9b28b18262c763608d60fba65e91fcb9ca3ddb3e))
    - first step towards reading the EOIE extension ([`068c716`](https://github.com/Byron/gitoxide/commit/068c716b46699234d6ad1db70be34b894e61d76a))
    - parse index header ([`5c731f8`](https://github.com/Byron/gitoxide/commit/5c731f831d007a4fe099cadc4ecaab113ab7e08a))
    - first stab at basic index file parsing ([`826ca0c`](https://github.com/Byron/gitoxide/commit/826ca0c6a6801ec2a67ca73ac17092e5f85fe9ce))
    - refactor ([`494ed46`](https://github.com/Byron/gitoxide/commit/494ed46acc54bd342f891416918032a2c4848cf1))
    - git-index uses memmap2 ([`fbfea28`](https://github.com/Byron/gitoxide/commit/fbfea28d2c9ed92e270c6a5aa603d3c84769ae8f))
    - The realization that FileBuffer really shouldn't be used anymore ([`b481f13`](https://github.com/Byron/gitoxide/commit/b481f136c4084b8839ebecb604dea5aa30d3a44e))
    - base setup for index testing ([`aa60fdf`](https://github.com/Byron/gitoxide/commit/aa60fdf3d86e08877c88f9e4973f546642ed1370))
    - notes on how test indices have been created ([`3040857`](https://github.com/Byron/gitoxide/commit/3040857ec4d2e0557b4920eaa77ddc4292d9adae))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade git-index->atoi to 1.0 ([`728dd65`](https://github.com/Byron/gitoxide/commit/728dd6501b86b12e1d0237256f94059a7ead14a9))
    - Use hash_hasher based hash state for better keys/less collisions ([`814de07`](https://github.com/Byron/gitoxide/commit/814de079f4226f42efa49ad334a348bce67184e4))
    - Also print stage of entries ([`003515f`](https://github.com/Byron/gitoxide/commit/003515f3c90a49fbe9db9b84987233486711beb8))
    - simple printing of basic entry information ([`329538b`](https://github.com/Byron/gitoxide/commit/329538b9c3f44bb8e70a4567ba90dc3b594c2dfc))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - Differentiate between owned and ref'ed path storage ([`c71b2bb`](https://github.com/Byron/gitoxide/commit/c71b2bb944c3066e7e44fbdd8a2e511a5a5d944a))
    - `State::path_backing()`. ([`8ab219a`](https://github.com/Byron/gitoxide/commit/8ab219ac47ca67f2478b8715d7820fd6171c0db2))
    - sketch `open_index()` on `Worktree`, but… ([`ff76261`](https://github.com/Byron/gitoxide/commit/ff76261f568f6b717a93b1f2dcf5d8e8b63acfca))
    - support for separating lifetimes of entries and path-backing ([`645ed50`](https://github.com/Byron/gitoxide/commit/645ed50dc2ae5ded2df0c09daf4fe366b90cf47e))
    - An attempt to build a lookup table of attribute files, but… ([`9841efb`](https://github.com/Byron/gitoxide/commit/9841efb566748dae6c79c5990c4fd1ecbc468aef))
    - refactor ([`475aa6a`](https://github.com/Byron/gitoxide/commit/475aa6a3e08f63df627a0988cd16c20494960c79))
    - Adjustments to support lower MSRV ([`16a0973`](https://github.com/Byron/gitoxide/commit/16a09737f0e81654cc7a5bbc9043385528524ca5))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - document-features support for git-index and git-worktree ([`1367cf5`](https://github.com/Byron/gitoxide/commit/1367cf5bc5908639e67e12f78f57835c5fd68a90))
    - make fmt ([`636fa8a`](https://github.com/Byron/gitoxide/commit/636fa8a97ce56982c76dffc64ee084e31d39afad))
    - strucural refactor ([`cdca1df`](https://github.com/Byron/gitoxide/commit/cdca1dfec590d24dd42f34294e21f4bdf61d36ad))
    - Allow mutation of entries during iteration, while obtaining their path ([`d0c4563`](https://github.com/Byron/gitoxide/commit/d0c4563f71ea18aaf8ae21dd8646ab256a550594))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - prevent line-ending conversions for shell scripts on windows ([`96bb4d4`](https://github.com/Byron/gitoxide/commit/96bb4d460db420e18dfd0f925109c740e971820d))
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - Assure we don't pick up unnecessary files during publishing ([`545b2d5`](https://github.com/Byron/gitoxide/commit/545b2d5121ba64efaee7564d5191cec37661efd7))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - fix docs ([`5a0d6b7`](https://github.com/Byron/gitoxide/commit/5a0d6b76205d6e021348a930a5a17820e5dc4458))
    - `Stage::entry_index_by_path_and_stage()`, now with `::entry_by_path_and_stage()` ([`6d8d5e6`](https://github.com/Byron/gitoxide/commit/6d8d5e6198dfb4d648061807ed4f96868a36ee52))
    - `State::entry_by_path_and_stage()` to find entries. ([`55363ea`](https://github.com/Byron/gitoxide/commit/55363ea650001b7717545b4d2968419707a3b8c6))
    - refactor; prepare for entry-lookup by path ([`92de081`](https://github.com/Byron/gitoxide/commit/92de081dc9ab5660cb18fa750452345dd63550ea))
    - `Debug` and `Clone` for `File` ([`40e6bde`](https://github.com/Byron/gitoxide/commit/40e6bde125778e3b50999331c4ed5a4b119937fa))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Clnoe for `File` ([`458e1bc`](https://github.com/Byron/gitoxide/commit/458e1bcbd7043f0759f7445bfa46189910baff54))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - add tests to run into long-paths special case ([`d7a8a7d`](https://github.com/Byron/gitoxide/commit/d7a8a7dfe3089e35fca249af7a3482a893f91111))
 * **Uncategorized**
    - Release gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`48f5bd2`](https://github.com/Byron/gitoxide/commit/48f5bd2014fa3dda6fbd60d091065c5537f69453))
    - Release gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`a5869e0`](https://github.com/Byron/gitoxide/commit/a5869e0b223406820bca836e3e3a7fae2bfd9b04))
    - Release gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`41d57b9`](https://github.com/Byron/gitoxide/commit/41d57b98964094fc1528adb09f69ca824229bf25))
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - fix link in docs for `sort_entries_by` ([`eb48b16`](https://github.com/Byron/gitoxide/commit/eb48b162c3f9c449de0fabfc2e8a33001a2d14d0))
    - rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - rename `git-index` to `gix-index` ([`ecd3541`](https://github.com/Byron/gitoxide/commit/ecd354119c3a8b150a06df7205ddf022a825d6cd))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - adjust to renaming of `git-lfs` to `gix-lfs` ([`b9225c8`](https://github.com/Byron/gitoxide/commit/b9225c830daf1388484ee7e05f727990fdeff43c))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-hashtable` to `gix-hashtable` ([`26a0c98`](https://github.com/Byron/gitoxide/commit/26a0c98d0a389b03e3dc7bfc758b37155e285244))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
    - Merge branch 'rename-crates' ([`6461c3d`](https://github.com/Byron/gitoxide/commit/6461c3da4d6daee857606d94294c3f87fc36965a))
    - rename `git-repository` to `gix` ([`7bed2a9`](https://github.com/Byron/gitoxide/commit/7bed2a96604397fa990f427b1a970ddeb6f09f1c))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - make fmt ([`e22080e`](https://github.com/Byron/gitoxide/commit/e22080e4a29d0bad15a99d565a5e3e304a8743ec))
    - Break cyclical dev dependencies ([`1fea18f`](https://github.com/Byron/gitoxide/commit/1fea18f5f8b4189a23dc4fa3f041a672f6fbcfb3))
    - make error handling of split index to seemingly be en-par with 'git' ([`cec2547`](https://github.com/Byron/gitoxide/commit/cec2547758b91fa340130d55a3920445b75b3636))
    - read shared indices by dissolving them into the current one. ([`5dc408f`](https://github.com/Byron/gitoxide/commit/5dc408f726d6f0f480438348eb5d713776329710))
    - inline bounds checks ([`dc3fd48`](https://github.com/Byron/gitoxide/commit/dc3fd48327a16bf8599eaf70b55ab6c1d754e4bb))
    - remove `VerifiedBitmaps` structure… ([`49cf105`](https://github.com/Byron/gitoxide/commit/49cf105329c7f84c314c11e50fdb039984fa94b3))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - move error handling into its own function ([`2b22d91`](https://github.com/Byron/gitoxide/commit/2b22d91f4b4f3aceab25cd54aa09aadff085d6ee))
    - Release git-index v0.12.1 ([`8aa5c1d`](https://github.com/Byron/gitoxide/commit/8aa5c1db9e342cc49dfa588d5b4b9f893067dbf7))
    - Merge branch 'various-improvements' ([`f191c1a`](https://github.com/Byron/gitoxide/commit/f191c1af14e512967ccb788fe997dafc5bb0e111))
    - add `impl Debug for State` to print all entries in a digestible form. ([`6d8eb9f`](https://github.com/Byron/gitoxide/commit/6d8eb9f267ac7ef8db0f3a277c8c991df5ce8164))
    - add `State::sort_entries_by(...)` to allow comparing by additional criteria. ([`7c8ba2c`](https://github.com/Byron/gitoxide/commit/7c8ba2c945d6313d27569f04b83ebf9a2387e6a2))
    - update documentation to make users of `State::dangerously_push_entry()` more aware. ([`b56de39`](https://github.com/Byron/gitoxide/commit/b56de392c632e9137091b7720208c38b162db7e7))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`0e3d0a5`](https://github.com/Byron/gitoxide/commit/0e3d0a56d7e6a60c6578138f2690b4fa54a2072d))
    - prepare changelogs prior to release ([`d679f5b`](https://github.com/Byron/gitoxide/commit/d679f5b6f018633e858d3ebbdaf1cd5098bbc5e7))
    - Merge branch 'various-improvements' ([`9eee8fe`](https://github.com/Byron/gitoxide/commit/9eee8fe1bed116b7a5a4f552982fa49da83ee92c))
    - `entry::Time` can convert from and to system time. ([`1e3341a`](https://github.com/Byron/gitoxide/commit/1e3341a77c089e6d80c271fca46b774b01b01386))
    - add `State::sort_entries()` and `State::dangerously_push_entry()`. ([`654bd8f`](https://github.com/Byron/gitoxide/commit/654bd8f62d5546b0f57e82d1be8211431685c7ce))
    - improved error handling - panicing version ([`9509891`](https://github.com/Byron/gitoxide/commit/9509891a63a0e2291818943366be6a085b9dd198))
    - add `State::entry_mut_by_path_and_stage()` ([`aa1b6ee`](https://github.com/Byron/gitoxide/commit/aa1b6ee1edeaebae1237184c635f69fdbb23ffee))
    - `State::write_to()` respects the `REMOVED` flag. ([`3ebe2d4`](https://github.com/Byron/gitoxide/commit/3ebe2d490b2dbdcb6fe0115b06f205f1c4008fff))
    - expose `git_hash` as `hash` in the root of the crate. ([`ec36586`](https://github.com/Byron/gitoxide/commit/ec365866c746247b7d5d47b88d51d9cc255724fb))
    - add `File::at_or_default(...)` to easily open or create an empty in-memory index. ([`5cc3a15`](https://github.com/Byron/gitoxide/commit/5cc3a15a8085a67701fc1f2d3ba0e55f71d0a4c0))
    - add `State::new()` to create a new empty in-memory index. ([`0b40951`](https://github.com/Byron/gitoxide/commit/0b40951f0fb9ee354a83751264ed89d0608111f8))
    - remove `File::into_state()` in favor of `From<File> for State`. ([`3753ad5`](https://github.com/Byron/gitoxide/commit/3753ad58a8303fbf325b07757b2b97c34253bca4))
    - add `File::set_path()` to allow setting the location of an index file on disk. ([`a7183e2`](https://github.com/Byron/gitoxide/commit/a7183e28513fa1e1a1a784b759677f0e4b4db5f4))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Assure recursive shared indices (possibly malicious) don't crash us ([`ede718c`](https://github.com/Byron/gitoxide/commit/ede718c24a5a5000bd41f49faabc7f2b19979e15))
    - refactor ([`d6ba366`](https://github.com/Byron/gitoxide/commit/d6ba3669f175e869060c97d359918e4901bde989))
    - refactor ([`c840c0d`](https://github.com/Byron/gitoxide/commit/c840c0d80aef8b4aadbd119108d2fb4c95a2503b))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - upload missing fixture archive ([`5cab553`](https://github.com/Byron/gitoxide/commit/5cab5530350c9d2792f3aea6cb8591a0602efee2))
    - fix failing test… ([`1057ee4`](https://github.com/Byron/gitoxide/commit/1057ee4309dfd44d9a160b799c0d704eaa8cce64))
    - refactor ([`dfd00e5`](https://github.com/Byron/gitoxide/commit/dfd00e52e288e4253527b3bd3559484acc45d3c6))
    - test is green ([`800fc2b`](https://github.com/Byron/gitoxide/commit/800fc2b7b6e802dacebf905ed72d5028d807758e))
    - sketch `resolve_link_extension` function ([`c62a25b`](https://github.com/Byron/gitoxide/commit/c62a25b9001cf6363314b6f90eab22b17ab7aff1))
    - add test that compares split index with regular index ([`fc89517`](https://github.com/Byron/gitoxide/commit/fc89517dffeb51ac56a33fdfe7ff1402136d488a))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - upgrade atoi from 1 to 2 ([`be6c65c`](https://github.com/Byron/gitoxide/commit/be6c65cb0fb056ae918b28050440946d0c2c9ada))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'named-threads' ([`726dd87`](https://github.com/Byron/gitoxide/commit/726dd87b5db45c333ccad898338a1cacea9e3269))
    - name spawned threads ([`fd2dd3a`](https://github.com/Byron/gitoxide/commit/fd2dd3a74c8d64407c1c27f29a2914389ded3bd6))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'main' into http-config ([`7c5b37d`](https://github.com/Byron/gitoxide/commit/7c5b37d28e98f59a6847368a0d0166d2dbb4acc1))
    - Release git-diff v0.22.0, git-index v0.7.1, git-pack v0.26.0, git-odb v0.36.0, git-transport v0.21.2, git-repository v0.27.0, safety bump 6 crates ([`f0cab31`](https://github.com/Byron/gitoxide/commit/f0cab317bb0c2799fa80d16f3ae1b89d6aee4284))
    - prepare changelogs prior to release ([`f5f3a9e`](https://github.com/Byron/gitoxide/commit/f5f3a9edd038a89c8c6c4da02054e5439bcc0071))
    - Merge branch 'fixes-for-crates-index-diff' ([`255be4d`](https://github.com/Byron/gitoxide/commit/255be4ddcd6cbca0a89f286eeecdd19ff70e000f))
    - remove unused import; fix docs ([`efe0a51`](https://github.com/Byron/gitoxide/commit/efe0a51931fc7e42c82563575e3068dd6e401409))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'write-sparse-index' ([`ba17db0`](https://github.com/Byron/gitoxide/commit/ba17db03e4e832db724ab3e08e3df05eb61dd401))
    - thanks clippy ([`49b539b`](https://github.com/Byron/gitoxide/commit/49b539baf1be88961a9e2934ee714090f94ac57f))
    - Remove tests and scaffolding code that probably won't be implemented soon. ([`177d1c8`](https://github.com/Byron/gitoxide/commit/177d1c8be2b73ab0e7534d8ba9a47c451e02cfbb))
    - refactor ([`0a74625`](https://github.com/Byron/gitoxide/commit/0a7462568c65057fb92b3824d0a73218c5184b2a))
    - Act like git and write a sparse index even if it contains no dir entries anymore. ([`53af48c`](https://github.com/Byron/gitoxide/commit/53af48cff26542b4acf1510862f7ac0e94b24b2b))
    - bake knowledge about sparse related config parameters into types. ([`e61957e`](https://github.com/Byron/gitoxide/commit/e61957e16eefe61d222997c69c1ae4c8ea0a8b5f))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - make fmt ([`ea2136b`](https://github.com/Byron/gitoxide/commit/ea2136b065979cecb3a1fdbf7b20ed7514128d9a))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - add and use `checked_is_sparse()` instead of cached `is_sparse` flag ([`e41ad0f`](https://github.com/Byron/gitoxide/commit/e41ad0fe585699b3d6cf3b3106567073e0a5ed5d))
    - refactor ([`3683963`](https://github.com/Byron/gitoxide/commit/36839630f1471bd73a13276652f3a6ddd1286faa))
    - thanks clippy ([`646b868`](https://github.com/Byron/gitoxide/commit/646b86802a669469b8cdfc228594a373a41e0f37))
    - added fixture, adjusted tests, refactor ([`3173c0b`](https://github.com/Byron/gitoxide/commit/3173c0b2f79fbba7d73e391cc5667ca35a56a3a1))
    - Make clear in code that mandatory extensions will always be written… ([`3e37443`](https://github.com/Byron/gitoxide/commit/3e3744301c3a80f98751551f779c3105262b3fec))
    - respect the current 'is_sparse()` state when writing. ([`2012b27`](https://github.com/Byron/gitoxide/commit/2012b27246e8835b19725862409d2df23a2638c6))
    - refactor ([`a929bcf`](https://github.com/Byron/gitoxide/commit/a929bcf4ff5a2e383218cce6b12776e40c553b83))
    - thanks clippy ([`5bfd947`](https://github.com/Byron/gitoxide/commit/5bfd94711568174afe3a344514745dcd6a4992a4))
    - sketch out how a write implementation could work ([`4a6d46f`](https://github.com/Byron/gitoxide/commit/4a6d46f3ab3d15eb851c92f7e49eb6772bc4023b))
    - regenerated archive ([`cd1c752`](https://github.com/Byron/gitoxide/commit/cd1c752fde943804689039684b60ae4ddffee3f1))
    - updated docs ([`77a9d42`](https://github.com/Byron/gitoxide/commit/77a9d42ec4f5b5f0b4fcbb31fdf2e5eb57bb578b))
    - added first tests and implementation for writing the `sdir` extension ([`66a675f`](https://github.com/Byron/gitoxide/commit/66a675f68e46e6eaf7464912d2fb8af976c18565))
    - capability to write `sdir`extension ([`762e4cb`](https://github.com/Byron/gitoxide/commit/762e4cb2a55728f6b82a97164c4ac4b59035d2e8))
    - added tests for reading sparse indexes ([`ddaa003`](https://github.com/Byron/gitoxide/commit/ddaa003246fce16578b455077d98519ac05c6dae))
    - add temporary sparse index playground testfile ([`5589a7f`](https://github.com/Byron/gitoxide/commit/5589a7fb4df6650214b7210bd89257ecaf9cabd0))
    - add sparse index text fixtures ([`8a8a53e`](https://github.com/Byron/gitoxide/commit/8a8a53e8432af7a96fd7eff9af0bd241c7b3facd))
    - add `is_sparse` access method for `State` ([`7f012cf`](https://github.com/Byron/gitoxide/commit/7f012cf5f85634bd520065ecb39bb1bd19a987fa))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - Merge branch 'fix-gix-index-from-tree' ([`da5f63c`](https://github.com/Byron/gitoxide/commit/da5f63cbc7506990f46d310f8064678decb86928))
    - `write::Options::object_hash` is now implied by the `State` itself. ([`59f6791`](https://github.com/Byron/gitoxide/commit/59f679126aba6f8a432aeb53f0bbd5d136ec1deb))
    - `decode::Options::object_hash` is now a parameter to methods. ([`908163a`](https://github.com/Byron/gitoxide/commit/908163ab2f86a1b603e69f04cd857fbf52e5abfb))
    - `decode::Options::default_from_object_hash()` ([`9e03110`](https://github.com/Byron/gitoxide/commit/9e03110578bd93da3f1a91c5bcd9fde942c81ac4))
    - refactor ([`6fb3255`](https://github.com/Byron/gitoxide/commit/6fb3255fb94758c025ed9edd971bdde54f409e77))
    - seal `File` members to preserve consistency better. ([`92dda50`](https://github.com/Byron/gitoxide/commit/92dda50e2d9c584b0e110026f59fb715ec41600a))
    - `decode::Options::default()` - remove `Default` impl. ([`bd312ac`](https://github.com/Byron/gitoxide/commit/bd312acf5ceba28edf2508aef6011c037eb0a377))
    - fix tests ([`fc5cee1`](https://github.com/Byron/gitoxide/commit/fc5cee1d4f757b634856b9d91df1b4455a63f860))
    - assure we also write V3 files, validate auto-version discovery ([`abc3cf8`](https://github.com/Byron/gitoxide/commit/abc3cf894f18f1a3c04de7967748add15b2e040e))
    - loose fixtures are usable more easily now ([`b86012b`](https://github.com/Byron/gitoxide/commit/b86012bd5407e67159c2cc86ce97525d92189284))
    - remove `write::Options::default()`. ([`2da5a62`](https://github.com/Byron/gitoxide/commit/2da5a62432350ede6b816254c894863d14aa4ba1))
    - `File::write()` for secure and complete writing of index files. ([`eedcffa`](https://github.com/Byron/gitoxide/commit/eedcffa728c7e895da51d5298db28f3fef05f7da))
    - prepare test for writing a complete index file from arbitrary state ([`281f5b8`](https://github.com/Byron/gitoxide/commit/281f5b828a2dbabd751e214b420e37d1d1e3a028))
    - Merge branch 'gix-index-from-tree' ([`8c24386`](https://github.com/Byron/gitoxide/commit/8c24386f1874cd94f78fefbe434963f772878b1f))
    - refactor ([`08d5c0b`](https://github.com/Byron/gitoxide/commit/08d5c0b051572b7e7b51eb7bd7dd804b1fa6a1ab))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - remove the .insert() call… ([`4bb3e8b`](https://github.com/Byron/gitoxide/commit/4bb3e8bd50958ddbfdee72247025a80a2ca850a8))
    - Merge branch 'main' into fetch-pack ([`d686020`](https://github.com/Byron/gitoxide/commit/d6860205db847b8a474756e92578195e1022481c))
    - thanks clippy ([`b9937ad`](https://github.com/Byron/gitoxide/commit/b9937adc2c31095dde63397be7d56f1ea559b0f7))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - make fmt ([`429cccc`](https://github.com/Byron/gitoxide/commit/429cccc5831c25a7205a12dc7a0443ac48616e2c))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - fix docs ([`87f6db7`](https://github.com/Byron/gitoxide/commit/87f6db7a7dc1561d06747135be206f700b75257c))
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - refactor ([`c40528e`](https://github.com/Byron/gitoxide/commit/c40528e86353fefe317d2b1ad33ff1236e589523))
    - refactor ([`b2835cc`](https://github.com/Byron/gitoxide/commit/b2835cc28e10907eb375b2beb400cf408fa5a3e0))
    - remove depthfirst traversal todo ([`5ca7945`](https://github.com/Byron/gitoxide/commit/5ca79458b11e0ead0027c64eecbc259b95f35ed5))
    - add test fixture and adjust ([`e153340`](https://github.com/Byron/gitoxide/commit/e153340f52bc13a980f347b215bc1337417bbbb4))
    - Overwrite duplicate entries (like 'git')… ([`16d8944`](https://github.com/Byron/gitoxide/commit/16d8944b492f9f70a2a29403f2ac1e1a3e50f450))
    - refactor ([`49dc4a6`](https://github.com/Byron/gitoxide/commit/49dc4a6967912df68bc4394f331b3a5242cf52e9))
    - refactor ([`c2524a6`](https://github.com/Byron/gitoxide/commit/c2524a635627449f5bcdd51b37a1dcd55ee0c193))
    - refactor ([`6683081`](https://github.com/Byron/gitoxide/commit/668308139e5981094ed2b059a97f1c1245b04dc1))
    - compare individual entries more thoroughly ([`1c9b703`](https://github.com/Byron/gitoxide/commit/1c9b703db10aa01e127f2a0eafe24f2aa1fefee7))
    - thanks clippy ([`878593e`](https://github.com/Byron/gitoxide/commit/878593e4d0a5e74df267ac7d0bdaf827b7588043))
    - refactor... ([`dce45e6`](https://github.com/Byron/gitoxide/commit/dce45e6ffdfe0031349bfff006e5e7140c2f515c))
    - :init module ([`6c17f96`](https://github.com/Byron/gitoxide/commit/6c17f96fcee9e2935b464c8ffbd30b253d9f5a6c))
    - refactor `Entry::cmp` ([`3a58c3e`](https://github.com/Byron/gitoxide/commit/3a58c3eb0b2802cd9acf05d8104a1b3a1dbc09bd))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - refactor ([`bba180d`](https://github.com/Byron/gitoxide/commit/bba180d78c182c873cc968bfd40186876dfde671))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - added more fixtures to test ([`adf5e54`](https://github.com/Byron/gitoxide/commit/adf5e5422a871ea435bb0ec320744b63f53d3159))
    - initial test and implementation for State::from_tree ([`14694a4`](https://github.com/Byron/gitoxide/commit/14694a4aeff7f05818aa7851e0e2fa56e911322c))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - prepare changelogs prior to release of git-testtools ([`7668e38`](https://github.com/Byron/gitoxide/commit/7668e38fab8891ed7e73fae3a6f5a8772e0f0d0b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - fix CI for good ([`e0c0b8c`](https://github.com/Byron/gitoxide/commit/e0c0b8c7c1898b2bc11a915e8e4fb8426295ccbb))
    - fix CI ([`2433be1`](https://github.com/Byron/gitoxide/commit/2433be173c2145198f7891dc7a1f7c4acf215b11))
    - Merge branch 'index-write-refactor' ([`805f432`](https://github.com/Byron/gitoxide/commit/805f432bf8e9d2dd9ede56caf959de386d5d80c7))
    - refactor ([`3af5121`](https://github.com/Byron/gitoxide/commit/3af5121330ae96aec32d0360c8b2e24a8860a2e8))
    - refactor ([`b41d93a`](https://github.com/Byron/gitoxide/commit/b41d93ac604b9807c24d93c6849f852489f512c0))
    - thanks clippy ([`4390c32`](https://github.com/Byron/gitoxide/commit/4390c32f9ea0683561a78349456c87329fef3b41))
    - run tests against all input files we have ([`de8abe6`](https://github.com/Byron/gitoxide/commit/de8abe6923b01563db812ba007ea65b7f193082d))
    - combine more tests into one to reduce duplication ([`933ad9e`](https://github.com/Byron/gitoxide/commit/933ad9e8ff0d58ad2590907cf84b43bc424e3219))
    - Assure that extended flags receive version 3; make `version` an implementation detail ([`6d810a1`](https://github.com/Byron/gitoxide/commit/6d810a135eeb71b8b04f7d9cb6c5f115587c2a63))
    - Support for extended flags, and V3 as it's a requirements. ([`417d90e`](https://github.com/Byron/gitoxide/commit/417d90eb267dd74a5372f1c7d8feb7cb4e98d2a1))
    - refcator ([`27993c0`](https://github.com/Byron/gitoxide/commit/27993c01a1533d561629635336c5cedf53dd0efc))
    - fix tree ext reading and writing; round-trip with long path works now ([`f93febe`](https://github.com/Byron/gitoxide/commit/f93febe2d2c55938ac8f698b57144583caab54ef))
    - first PoC for writing long paths, even though it doens't produce the entire file yet ([`581cbd7`](https://github.com/Byron/gitoxide/commit/581cbd7afeac0f7654611c83deacae802ef5da6f))
    - Make it more explicit to write all available extensions by default ([`fbe9815`](https://github.com/Byron/gitoxide/commit/fbe981519446e55c4020e95841e7bff7e54e358e))
    - fix docs ([`9861a6c`](https://github.com/Byron/gitoxide/commit/9861a6ce8abc438a1e0739aa6d55ced450a4465b))
    - thanks clippy ([`834be93`](https://github.com/Byron/gitoxide/commit/834be93e6db84bb9160dd4677b7e9d63213c23cd))
    - thanks clippy ([`9b3a940`](https://github.com/Byron/gitoxide/commit/9b3a940d9f4694912f32cb86752f3f7507882010))
    - generalize extension writing so that writing more will be easier ([`8ef5378`](https://github.com/Byron/gitoxide/commit/8ef5378dfaefe2d562d16b861fb4bb0fa4fdfe93))
    - generalize EOIE exstension writing ([`18b722e`](https://github.com/Byron/gitoxide/commit/18b722e06bfb8bbbf0ada7438266e31a4317f2d4))
    - provide a stand-alone way of writing end-of-index extensions ([`7ca297a`](https://github.com/Byron/gitoxide/commit/7ca297af400e50d42cffcaab54b1684f6810eb4f))
    - refactor ([`a5b2ef9`](https://github.com/Byron/gitoxide/commit/a5b2ef9a33720312a6b30b7cdae564bf759b0218))
    - additional validation ([`ee7b5bb`](https://github.com/Byron/gitoxide/commit/ee7b5bba09bc20e1531cb733b5b2aac8232e7674))
    - refactor ([`e35aac6`](https://github.com/Byron/gitoxide/commit/e35aac66079464a9494744c201355ab2faa0a2b3))
    - refactor ([`52386f4`](https://github.com/Byron/gitoxide/commit/52386f4a8ee11b0d2858412b4b5ec4b73544ba30))
    - refactor ([`75a2338`](https://github.com/Byron/gitoxide/commit/75a2338fe9cfac478164b7b575c0f3c2b910111d))
    - refactor ([`f6f2861`](https://github.com/Byron/gitoxide/commit/f6f2861f57be8ad4c795c90ae6fc7e568aeb12da))
    - refactor ([`6cf9277`](https://github.com/Byron/gitoxide/commit/6cf92776b0349bf735c28b6275fdf551ce236d4d))
    - refactor ([`a6354c0`](https://github.com/Byron/gitoxide/commit/a6354c076b2967ff31feb30e7b73f0a6eb92a459))
    - Fill in all remaining documentation, raise `git-index` to 'usable' state ([`3568ae3`](https://github.com/Byron/gitoxide/commit/3568ae3a1ed7c2d0c9b7e1dc690b055b4f43bdd2))
    - first step towards everything being documented ([`919923c`](https://github.com/Byron/gitoxide/commit/919923c08b641ca148c2f25d193d65bb068cc787))
    - remove quickerror in favor of thiserror ([`dd7ce3f`](https://github.com/Byron/gitoxide/commit/dd7ce3f77c868f81196103b021957ace54ca2b9c))
    - refactor ([`618736b`](https://github.com/Byron/gitoxide/commit/618736b614330d9576e58c5bb9b3696de3f76d84))
    - refactor ([`4dda27e`](https://github.com/Byron/gitoxide/commit/4dda27e716927a506e16da9d6cd50547de1fc84e))
    - added test for eoie extension ([`a433c0d`](https://github.com/Byron/gitoxide/commit/a433c0d7feebebdf17e9d362894a6fa38221b402))
    - estimate vector size for tree entries ([`74455e6`](https://github.com/Byron/gitoxide/commit/74455e65e3ef48ce55fa40b7d9ef050a8e0e9b84))
    - implemented File::write_to for hashed write ([`6b6db34`](https://github.com/Byron/gitoxide/commit/6b6db340b351836f055d8ab74ed3d3f370cca7be))
    - refactor and add more tests ([`6b32bcf`](https://github.com/Byron/gitoxide/commit/6b32bcfde1af3ada67a6fa0448611d0ecca2f605))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - thanks clippy ([`a66403c`](https://github.com/Byron/gitoxide/commit/a66403c8e14716023455d606e1c63787ac40f4f4))
    - write wrapper to count written bytes ([`b147090`](https://github.com/Byron/gitoxide/commit/b147090bafd22da07f475a167bb921f3e0fa0017))
    - refactor test ([`33a3009`](https://github.com/Byron/gitoxide/commit/33a3009c4d851fae9156cb3bdb664c05118ef442))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - refactor... ([`81eef35`](https://github.com/Byron/gitoxide/commit/81eef353f8f2add26720e3dd3981ce1b790f996c))
    - refactor tests... ([`3a9b51b`](https://github.com/Byron/gitoxide/commit/3a9b51b0fe812d454378c8ac887bba11740a81ee))
    - convert 'in-memory' flags to 'storage' flags ([`017377d`](https://github.com/Byron/gitoxide/commit/017377d8c49c236bb3ab240794dbd7a714efb7e1))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - small improvements ([`e5cb6d9`](https://github.com/Byron/gitoxide/commit/e5cb6d94ef31e007847a6072ead8962b16eba105))
    - fix pathname in test ([`1f18e19`](https://github.com/Byron/gitoxide/commit/1f18e19fd3c07b540d56c86afa4cb708ad1126ac))
    - thanks clippy ([`3f72180`](https://github.com/Byron/gitoxide/commit/3f7218044fdc9d24693c04e0c1c97069c9a3f698))
    - sucesfully writing the first basic index files ([`a9c6f22`](https://github.com/Byron/gitoxide/commit/a9c6f2260e96928d678b29a765c26e88f0ff5908))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - setup and refactor tests ([`7eed237`](https://github.com/Byron/gitoxide/commit/7eed2375a3f076e6fdf9dde4673733e85d5612aa))
    - generate index header ([`f1d7c1c`](https://github.com/Byron/gitoxide/commit/f1d7c1c137c712ecca76b8e69b11481bf0f1a860))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Merge branch 'worktree-stack' ([`39046e9`](https://github.com/Byron/gitoxide/commit/39046e98098da7d490757477986479126a45b3e5))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Implemented git-worktree ([`4177d72`](https://github.com/Byron/gitoxide/commit/4177d72c95bd94cf6a49e917dc21918044e8250b))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - refactor ([`afdeca1`](https://github.com/Byron/gitoxide/commit/afdeca1e5ec119607c5d1f5ccec5d216fc7d5261))
    - thanks clippy ([`2f25bf1`](https://github.com/Byron/gitoxide/commit/2f25bf1ebf44aef8c4886eaefb3e87836d535f61))
    - thanks clippy ([`d721019`](https://github.com/Byron/gitoxide/commit/d721019aebe5b01ddb15c9b1aab279647069452a))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - upgrade to tui 0.17 and prodash 18 ([`eba101a`](https://github.com/Byron/gitoxide/commit/eba101a576ecb7bc0f63357d0dd81eb817b94be4))
    - dependency update ([`ca59e44`](https://github.com/Byron/gitoxide/commit/ca59e448061698dd559db43123fe676ae61129a0))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - thanks clippy ([`09df2bc`](https://github.com/Byron/gitoxide/commit/09df2bcb4b45f72742d139530907be8aa4dc36f8))
    - thanks clippy ([`93c3d23`](https://github.com/Byron/gitoxide/commit/93c3d23d255a02d65b5404c2f62f96d94e36f33d))
    - Fix index without extension test & thanks clippy ([`066464d`](https://github.com/Byron/gitoxide/commit/066464d2ad2833012fa196fe41e93a54ab05457f))
    - thanks clippy ([`f477032`](https://github.com/Byron/gitoxide/commit/f47703256fe6a5c68ed3af6705bcdf01262500d6))
    - thanks clippy ([`5526020`](https://github.com/Byron/gitoxide/commit/552602074a99dc536624f0c6295e807caf32f58b))
    - thanks clippy ([`591511a`](https://github.com/Byron/gitoxide/commit/591511a739f91c5e8ff4243059ac98052a44c914))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`e4bcfe6`](https://github.com/Byron/gitoxide/commit/e4bcfe6406b14feffa63598c7cdcc8ecc73222bd))
    - add placeholder for git-index crate ([`52ff13c`](https://github.com/Byron/gitoxide/commit/52ff13cf260b53423faf59e5c666ff1565bde947))
</details>

## 0.12.2 (2023-01-10)

A maintenance release without user-facing changes.

## 0.12.1 (2023-01-08)

### New Features

 - <csr-id-6d8eb9f267ac7ef8db0f3a277c8c991df5ce8164/> add `impl Debug for State` to print all entries in a digestible form.
 - <csr-id-7c8ba2c945d6313d27569f04b83ebf9a2387e6a2/> add `State::sort_entries_by(...)` to allow comparing by additional criteria.
   This allows to organize entries based on flags, for example, which may help
   in special occasions.

## 0.12.0 (2023-01-06)

### New Features

 - <csr-id-1e3341a77c089e6d80c271fca46b774b01b01386/> `entry::Time` can convert from and to system time.
 - <csr-id-654bd8f62d5546b0f57e82d1be8211431685c7ce/> add `State::sort_entries()` and `State::dangerously_push_entry()`.
   Both methods work in tandem as one breaks invariants, but allows to quickly
   add entries, while the other restores them.
 - <csr-id-aa1b6ee1edeaebae1237184c635f69fdbb23ffee/> add `State::entry_mut_by_path_and_stage()`
 - <csr-id-3ebe2d490b2dbdcb6fe0115b06f205f1c4008fff/> `State::write_to()` respects the `REMOVED` flag.
   That way, one can mark entries for removal and these will be pruned
   at write time. This is preferable over performing removals expensively
   in memory.
 - <csr-id-ec365866c746247b7d5d47b88d51d9cc255724fb/> expose `gix_hash` as `hash` in the root of the crate.
   This makes it easier to use the crate standalone as plumbing as instantiation
   requires access to `gix_hash`.
 - <csr-id-5cc3a15a8085a67701fc1f2d3ba0e55f71d0a4c0/> add `File::at_or_default(...)` to easily open or create an empty in-memory index.
   This is a common operation in new repositories.
 - <csr-id-0b40951f0fb9ee354a83751264ed89d0608111f8/> add `State::new()` to create a new empty in-memory index.
 - <csr-id-a7183e28513fa1e1a1a784b759677f0e4b4db5f4/> add `File::set_path()` to allow setting the location of an index file on disk.
   This is useful to change the location of an index after reading it (from another
   location, similar to copy-on-write).

### Changed (BREAKING)

 - <csr-id-3753ad58a8303fbf325b07757b2b97c34253bca4/> remove `File::into_state()` in favor of `From<File> for State`.
   That way it's less discoverable, but more idiomatic, and we don't want to
   get into the habit of providing multiple names of the exact same
   functionality.

## 0.11.0 (2022-12-30)

A maintenance release without user-facing changes.

## 0.10.0 (2022-12-19)

A maintenance release without user-facing changes.

## 0.9.1 (2022-11-27)

### New Features

 - <csr-id-fd2dd3a74c8d64407c1c27f29a2914389ded3bd6/> name spawned threads
   That way it's a bit more obvious what's happening when the CPU goes
   up in flames.

## 0.9.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.8.0 (2022-11-17)

A maintenance release without user-facing changes.

## 0.7.1 (2022-11-08)

A maintenance release without user-facing changes.

## 0.7.0 (2022-11-06)

<csr-id-4a6d46f3ab3d15eb851c92f7e49eb6772bc4023b/>

### New Features

 - <csr-id-458e1bcbd7043f0759f7445bfa46189910baff54/> Clone for `File`
 - <csr-id-9e03110578bd93da3f1a91c5bcd9fde942c81ac4/> `decode::Options::default_from_object_hash()`
   An easier way to initialize decode options, providing only the mandatory
   information.
 - <csr-id-eedcffa728c7e895da51d5298db28f3fef05f7da/> `File::write()` for secure and complete writing of index files.

### Other

 - <csr-id-4a6d46f3ab3d15eb851c92f7e49eb6772bc4023b/> sketch out how a write implementation could work

### Changed (BREAKING)

 - <csr-id-59f679126aba6f8a432aeb53f0bbd5d136ec1deb/> `write::Options::object_hash` is now implied by the `State` itself.
   The `State`, once initialized, knows the kind of object hash it uses and
   there is no need to specify it again.
   
   This affects some method signatures which now work without
   `object_hash`.
 - <csr-id-908163ab2f86a1b603e69f04cd857fbf52e5abfb/> `decode::Options::object_hash` is now a parameter to methods.
   It's not actually an option that could be defaulted, but an integral
   piece of knowledge that must always be defined by the caller.
   
   This also makes `decode::Options::default()` available once again.
 - <csr-id-92dda50e2d9c584b0e110026f59fb715ec41600a/> seal `File` members to preserve consistency better.
   This also makes sure that it's obvious if the `checksum` is actually
   already computed.

### Reverted (BREAKING)

 - <csr-id-bd312acf5ceba28edf2508aef6011c037eb0a377/> `decode::Options::default()` - remove `Default` impl.
   The contained `gix_hash::Kind` can't actually be defaulted as we
   have to know the actual kind used in the repository.
 - <csr-id-2da5a62432350ede6b816254c894863d14aa4ba1/> remove `write::Options::default()`.
   In practice it's required to inform about the hash kind to use and it's
   possibly incorrect to assume Sha1.

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

## 0.5.0 (2022-09-20)

<csr-id-6c17f96fcee9e2935b464c8ffbd30b253d9f5a6c/>

### Other

 - <csr-id-6c17f96fcee9e2935b464c8ffbd30b253d9f5a6c/> :init module

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.3 (2022-08-27)

Maintenance release without user-facing changes.

## 0.4.2 (2022-08-24)

<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

## 0.4.1 (2022-08-17)

### New Features

 - <csr-id-6d8d5e6198dfb4d648061807ed4f96868a36ee52/> `Stage::entry_index_by_path_and_stage()`, now with `::entry_by_path_and_stage()`
 - <csr-id-55363ea650001b7717545b4d2968419707a3b8c6/> `State::entry_by_path_and_stage()` to find entries.
 - <csr-id-40e6bde125778e3b50999331c4ed5a4b119937fa/> `Debug` and `Clone` for `File`

## 0.4.0 (2022-07-22)

This is a maintenance release with no functional changes.

## 0.3.0 (2022-05-18)

### New Features

 - <csr-id-8ab219ac47ca67f2478b8715d7820fd6171c0db2/> `State::path_backing()`.
   That way it's possible to call certain methods that take a separate
   path buffer.
 - <csr-id-645ed50dc2ae5ded2df0c09daf4fe366b90cf47e/> support for separating lifetimes of entries and path-backing
   This way it should be possible to access paths immutably even while
   entries are available mutably, assuming we stagger accesses to put
   mutation of entries last.

## 0.2.0 (2022-04-03)

### Bug Fixes

 - <csr-id-c2cc939d131a278c177c5f44d3b26127c65bd352/> lower MSRV to 1.52

### Bug Fixes (BREAKING)

 - <csr-id-0b1543d481337ed51dcfdeb907af21f0bc98dcb9/> lower rust edition to 2018

## 0.1.0 (2022-01-19)

The initial release which can read a complete index, version 2 to 4, with all extensions.
The reading can be performed with multiple threads as well, partially depending on whether
certain extensions are present.

## v0.0.0 (2020-08-28)

