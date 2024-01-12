//! Pre-computed tables powers-of-5 for extended-precision representations.
//!
//! These tables enable fast scaling of the significant digits
//! of a float to the decimal exponent, with minimal rounding
//! errors, in a 128 or 192-bit representation.
//!
//! DO NOT MODIFY: Generated by `src/etc/dec2flt_table.py`

pub(crate) const SMALLEST_POWER_OF_FIVE: i32 = -342;
pub(crate) const LARGEST_POWER_OF_FIVE: i32 = 308;
pub(crate) const N_POWERS_OF_FIVE: usize =
    (LARGEST_POWER_OF_FIVE - SMALLEST_POWER_OF_FIVE + 1) as usize;

// Use static to avoid long compile times: Rust compiler errors
// can have the entire table compiled multiple times, and then
// emit code multiple times, even if it's stripped out in
// the final binary.
#[rustfmt::skip]
pub(crate) static POWER_OF_FIVE_128: [(u64, u64); N_POWERS_OF_FIVE] = [
    (0xeef453d6923bd65a, 0x113faa2906a13b3f), // 5^-342
    (0x9558b4661b6565f8, 0x4ac7ca59a424c507), // 5^-341
    (0xbaaee17fa23ebf76, 0x5d79bcf00d2df649), // 5^-340
    (0xe95a99df8ace6f53, 0xf4d82c2c107973dc), // 5^-339
    (0x91d8a02bb6c10594, 0x79071b9b8a4be869), // 5^-338
    (0xb64ec836a47146f9, 0x9748e2826cdee284), // 5^-337
    (0xe3e27a444d8d98b7, 0xfd1b1b2308169b25), // 5^-336
    (0x8e6d8c6ab0787f72, 0xfe30f0f5e50e20f7), // 5^-335
    (0xb208ef855c969f4f, 0xbdbd2d335e51a935), // 5^-334
    (0xde8b2b66b3bc4723, 0xad2c788035e61382), // 5^-333
    (0x8b16fb203055ac76, 0x4c3bcb5021afcc31), // 5^-332
    (0xaddcb9e83c6b1793, 0xdf4abe242a1bbf3d), // 5^-331
    (0xd953e8624b85dd78, 0xd71d6dad34a2af0d), // 5^-330
    (0x87d4713d6f33aa6b, 0x8672648c40e5ad68), // 5^-329
    (0xa9c98d8ccb009506, 0x680efdaf511f18c2), // 5^-328
    (0xd43bf0effdc0ba48, 0x212bd1b2566def2),  // 5^-327
    (0x84a57695fe98746d, 0x14bb630f7604b57),  // 5^-326
    (0xa5ced43b7e3e9188, 0x419ea3bd35385e2d), // 5^-325
    (0xcf42894a5dce35ea, 0x52064cac828675b9), // 5^-324
    (0x818995ce7aa0e1b2, 0x7343efebd1940993), // 5^-323
    (0xa1ebfb4219491a1f, 0x1014ebe6c5f90bf8), // 5^-322
    (0xca66fa129f9b60a6, 0xd41a26e077774ef6), // 5^-321
    (0xfd00b897478238d0, 0x8920b098955522b4), // 5^-320
    (0x9e20735e8cb16382, 0x55b46e5f5d5535b0), // 5^-319
    (0xc5a890362fddbc62, 0xeb2189f734aa831d), // 5^-318
    (0xf712b443bbd52b7b, 0xa5e9ec7501d523e4), // 5^-317
    (0x9a6bb0aa55653b2d, 0x47b233c92125366e), // 5^-316
    (0xc1069cd4eabe89f8, 0x999ec0bb696e840a), // 5^-315
    (0xf148440a256e2c76, 0xc00670ea43ca250d), // 5^-314
    (0x96cd2a865764dbca, 0x380406926a5e5728), // 5^-313
    (0xbc807527ed3e12bc, 0xc605083704f5ecf2), // 5^-312
    (0xeba09271e88d976b, 0xf7864a44c633682e), // 5^-311
    (0x93445b8731587ea3, 0x7ab3ee6afbe0211d), // 5^-310
    (0xb8157268fdae9e4c, 0x5960ea05bad82964), // 5^-309
    (0xe61acf033d1a45df, 0x6fb92487298e33bd), // 5^-308
    (0x8fd0c16206306bab, 0xa5d3b6d479f8e056), // 5^-307
    (0xb3c4f1ba87bc8696, 0x8f48a4899877186c), // 5^-306
    (0xe0b62e2929aba83c, 0x331acdabfe94de87), // 5^-305
    (0x8c71dcd9ba0b4925, 0x9ff0c08b7f1d0b14), // 5^-304
    (0xaf8e5410288e1b6f, 0x7ecf0ae5ee44dd9),  // 5^-303
    (0xdb71e91432b1a24a, 0xc9e82cd9f69d6150), // 5^-302
    (0x892731ac9faf056e, 0xbe311c083a225cd2), // 5^-301
    (0xab70fe17c79ac6ca, 0x6dbd630a48aaf406), // 5^-300
    (0xd64d3d9db981787d, 0x92cbbccdad5b108),  // 5^-299
    (0x85f0468293f0eb4e, 0x25bbf56008c58ea5), // 5^-298
    (0xa76c582338ed2621, 0xaf2af2b80af6f24e), // 5^-297
    (0xd1476e2c07286faa, 0x1af5af660db4aee1), // 5^-296
    (0x82cca4db847945ca, 0x50d98d9fc890ed4d), // 5^-295
    (0xa37fce126597973c, 0xe50ff107bab528a0), // 5^-294
    (0xcc5fc196fefd7d0c, 0x1e53ed49a96272c8), // 5^-293
    (0xff77b1fcbebcdc4f, 0x25e8e89c13bb0f7a), // 5^-292
    (0x9faacf3df73609b1, 0x77b191618c54e9ac), // 5^-291
    (0xc795830d75038c1d, 0xd59df5b9ef6a2417), // 5^-290
    (0xf97ae3d0d2446f25, 0x4b0573286b44ad1d), // 5^-289
    (0x9becce62836ac577, 0x4ee367f9430aec32), // 5^-288
    (0xc2e801fb244576d5, 0x229c41f793cda73f), // 5^-287
    (0xf3a20279ed56d48a, 0x6b43527578c1110f), // 5^-286
    (0x9845418c345644d6, 0x830a13896b78aaa9), // 5^-285
    (0xbe5691ef416bd60c, 0x23cc986bc656d553), // 5^-284
    (0xedec366b11c6cb8f, 0x2cbfbe86b7ec8aa8), // 5^-283
    (0x94b3a202eb1c3f39, 0x7bf7d71432f3d6a9), // 5^-282
    (0xb9e08a83a5e34f07, 0xdaf5ccd93fb0cc53), // 5^-281
    (0xe858ad248f5c22c9, 0xd1b3400f8f9cff68), // 5^-280
    (0x91376c36d99995be, 0x23100809b9c21fa1), // 5^-279
    (0xb58547448ffffb2d, 0xabd40a0c2832a78a), // 5^-278
    (0xe2e69915b3fff9f9, 0x16c90c8f323f516c), // 5^-277
    (0x8dd01fad907ffc3b, 0xae3da7d97f6792e3), // 5^-276
    (0xb1442798f49ffb4a, 0x99cd11cfdf41779c), // 5^-275
    (0xdd95317f31c7fa1d, 0x40405643d711d583), // 5^-274
    (0x8a7d3eef7f1cfc52, 0x482835ea666b2572), // 5^-273
    (0xad1c8eab5ee43b66, 0xda3243650005eecf), // 5^-272
    (0xd863b256369d4a40, 0x90bed43e40076a82), // 5^-271
    (0x873e4f75e2224e68, 0x5a7744a6e804a291), // 5^-270
    (0xa90de3535aaae202, 0x711515d0a205cb36), // 5^-269
    (0xd3515c2831559a83, 0xd5a5b44ca873e03),  // 5^-268
    (0x8412d9991ed58091, 0xe858790afe9486c2), // 5^-267
    (0xa5178fff668ae0b6, 0x626e974dbe39a872), // 5^-266
    (0xce5d73ff402d98e3, 0xfb0a3d212dc8128f), // 5^-265
    (0x80fa687f881c7f8e, 0x7ce66634bc9d0b99), // 5^-264
    (0xa139029f6a239f72, 0x1c1fffc1ebc44e80), // 5^-263
    (0xc987434744ac874e, 0xa327ffb266b56220), // 5^-262
    (0xfbe9141915d7a922, 0x4bf1ff9f0062baa8), // 5^-261
    (0x9d71ac8fada6c9b5, 0x6f773fc3603db4a9), // 5^-260
    (0xc4ce17b399107c22, 0xcb550fb4384d21d3), // 5^-259
    (0xf6019da07f549b2b, 0x7e2a53a146606a48), // 5^-258
    (0x99c102844f94e0fb, 0x2eda7444cbfc426d), // 5^-257
    (0xc0314325637a1939, 0xfa911155fefb5308), // 5^-256
    (0xf03d93eebc589f88, 0x793555ab7eba27ca), // 5^-255
    (0x96267c7535b763b5, 0x4bc1558b2f3458de), // 5^-254
    (0xbbb01b9283253ca2, 0x9eb1aaedfb016f16), // 5^-253
    (0xea9c227723ee8bcb, 0x465e15a979c1cadc), // 5^-252
    (0x92a1958a7675175f, 0xbfacd89ec191ec9),  // 5^-251
    (0xb749faed14125d36, 0xcef980ec671f667b), // 5^-250
    (0xe51c79a85916f484, 0x82b7e12780e7401a), // 5^-249
    (0x8f31cc0937ae58d2, 0xd1b2ecb8b0908810), // 5^-248
    (0xb2fe3f0b8599ef07, 0x861fa7e6dcb4aa15), // 5^-247
    (0xdfbdcece67006ac9, 0x67a791e093e1d49a), // 5^-246
    (0x8bd6a141006042bd, 0xe0c8bb2c5c6d24e0), // 5^-245
    (0xaecc49914078536d, 0x58fae9f773886e18), // 5^-244
    (0xda7f5bf590966848, 0xaf39a475506a899e), // 5^-243
    (0x888f99797a5e012d, 0x6d8406c952429603), // 5^-242
    (0xaab37fd7d8f58178, 0xc8e5087ba6d33b83), // 5^-241
    (0xd5605fcdcf32e1d6, 0xfb1e4a9a90880a64), // 5^-240
    (0x855c3be0a17fcd26, 0x5cf2eea09a55067f), // 5^-239
    (0xa6b34ad8c9dfc06f, 0xf42faa48c0ea481e), // 5^-238
    (0xd0601d8efc57b08b, 0xf13b94daf124da26), // 5^-237
    (0x823c12795db6ce57, 0x76c53d08d6b70858), // 5^-236
    (0xa2cb1717b52481ed, 0x54768c4b0c64ca6e), // 5^-235
    (0xcb7ddcdda26da268, 0xa9942f5dcf7dfd09), // 5^-234
    (0xfe5d54150b090b02, 0xd3f93b35435d7c4c), // 5^-233
    (0x9efa548d26e5a6e1, 0xc47bc5014a1a6daf), // 5^-232
    (0xc6b8e9b0709f109a, 0x359ab6419ca1091b), // 5^-231
    (0xf867241c8cc6d4c0, 0xc30163d203c94b62), // 5^-230
    (0x9b407691d7fc44f8, 0x79e0de63425dcf1d), // 5^-229
    (0xc21094364dfb5636, 0x985915fc12f542e4), // 5^-228
    (0xf294b943e17a2bc4, 0x3e6f5b7b17b2939d), // 5^-227
    (0x979cf3ca6cec5b5a, 0xa705992ceecf9c42), // 5^-226
    (0xbd8430bd08277231, 0x50c6ff782a838353), // 5^-225
    (0xece53cec4a314ebd, 0xa4f8bf5635246428), // 5^-224
    (0x940f4613ae5ed136, 0x871b7795e136be99), // 5^-223
    (0xb913179899f68584, 0x28e2557b59846e3f), // 5^-222
    (0xe757dd7ec07426e5, 0x331aeada2fe589cf), // 5^-221
    (0x9096ea6f3848984f, 0x3ff0d2c85def7621), // 5^-220
    (0xb4bca50b065abe63, 0xfed077a756b53a9),  // 5^-219
    (0xe1ebce4dc7f16dfb, 0xd3e8495912c62894), // 5^-218
    (0x8d3360f09cf6e4bd, 0x64712dd7abbbd95c), // 5^-217
    (0xb080392cc4349dec, 0xbd8d794d96aacfb3), // 5^-216
    (0xdca04777f541c567, 0xecf0d7a0fc5583a0), // 5^-215
    (0x89e42caaf9491b60, 0xf41686c49db57244), // 5^-214
    (0xac5d37d5b79b6239, 0x311c2875c522ced5), // 5^-213
    (0xd77485cb25823ac7, 0x7d633293366b828b), // 5^-212
    (0x86a8d39ef77164bc, 0xae5dff9c02033197), // 5^-211
    (0xa8530886b54dbdeb, 0xd9f57f830283fdfc), // 5^-210
    (0xd267caa862a12d66, 0xd072df63c324fd7b), // 5^-209
    (0x8380dea93da4bc60, 0x4247cb9e59f71e6d), // 5^-208
    (0xa46116538d0deb78, 0x52d9be85f074e608), // 5^-207
    (0xcd795be870516656, 0x67902e276c921f8b), // 5^-206
    (0x806bd9714632dff6, 0xba1cd8a3db53b6),   // 5^-205
    (0xa086cfcd97bf97f3, 0x80e8a40eccd228a4), // 5^-204
    (0xc8a883c0fdaf7df0, 0x6122cd128006b2cd), // 5^-203
    (0xfad2a4b13d1b5d6c, 0x796b805720085f81), // 5^-202
    (0x9cc3a6eec6311a63, 0xcbe3303674053bb0), // 5^-201
    (0xc3f490aa77bd60fc, 0xbedbfc4411068a9c), // 5^-200
    (0xf4f1b4d515acb93b, 0xee92fb5515482d44), // 5^-199
    (0x991711052d8bf3c5, 0x751bdd152d4d1c4a), // 5^-198
    (0xbf5cd54678eef0b6, 0xd262d45a78a0635d), // 5^-197
    (0xef340a98172aace4, 0x86fb897116c87c34), // 5^-196
    (0x9580869f0e7aac0e, 0xd45d35e6ae3d4da0), // 5^-195
    (0xbae0a846d2195712, 0x8974836059cca109), // 5^-194
    (0xe998d258869facd7, 0x2bd1a438703fc94b), // 5^-193
    (0x91ff83775423cc06, 0x7b6306a34627ddcf), // 5^-192
    (0xb67f6455292cbf08, 0x1a3bc84c17b1d542), // 5^-191
    (0xe41f3d6a7377eeca, 0x20caba5f1d9e4a93), // 5^-190
    (0x8e938662882af53e, 0x547eb47b7282ee9c), // 5^-189
    (0xb23867fb2a35b28d, 0xe99e619a4f23aa43), // 5^-188
    (0xdec681f9f4c31f31, 0x6405fa00e2ec94d4), // 5^-187
    (0x8b3c113c38f9f37e, 0xde83bc408dd3dd04), // 5^-186
    (0xae0b158b4738705e, 0x9624ab50b148d445), // 5^-185
    (0xd98ddaee19068c76, 0x3badd624dd9b0957), // 5^-184
    (0x87f8a8d4cfa417c9, 0xe54ca5d70a80e5d6), // 5^-183
    (0xa9f6d30a038d1dbc, 0x5e9fcf4ccd211f4c), // 5^-182
    (0xd47487cc8470652b, 0x7647c3200069671f), // 5^-181
    (0x84c8d4dfd2c63f3b, 0x29ecd9f40041e073), // 5^-180
    (0xa5fb0a17c777cf09, 0xf468107100525890), // 5^-179
    (0xcf79cc9db955c2cc, 0x7182148d4066eeb4), // 5^-178
    (0x81ac1fe293d599bf, 0xc6f14cd848405530), // 5^-177
    (0xa21727db38cb002f, 0xb8ada00e5a506a7c), // 5^-176
    (0xca9cf1d206fdc03b, 0xa6d90811f0e4851c), // 5^-175
    (0xfd442e4688bd304a, 0x908f4a166d1da663), // 5^-174
    (0x9e4a9cec15763e2e, 0x9a598e4e043287fe), // 5^-173
    (0xc5dd44271ad3cdba, 0x40eff1e1853f29fd), // 5^-172
    (0xf7549530e188c128, 0xd12bee59e68ef47c), // 5^-171
    (0x9a94dd3e8cf578b9, 0x82bb74f8301958ce), // 5^-170
    (0xc13a148e3032d6e7, 0xe36a52363c1faf01), // 5^-169
    (0xf18899b1bc3f8ca1, 0xdc44e6c3cb279ac1), // 5^-168
    (0x96f5600f15a7b7e5, 0x29ab103a5ef8c0b9), // 5^-167
    (0xbcb2b812db11a5de, 0x7415d448f6b6f0e7), // 5^-166
    (0xebdf661791d60f56, 0x111b495b3464ad21), // 5^-165
    (0x936b9fcebb25c995, 0xcab10dd900beec34), // 5^-164
    (0xb84687c269ef3bfb, 0x3d5d514f40eea742), // 5^-163
    (0xe65829b3046b0afa, 0xcb4a5a3112a5112),  // 5^-162
    (0x8ff71a0fe2c2e6dc, 0x47f0e785eaba72ab), // 5^-161
    (0xb3f4e093db73a093, 0x59ed216765690f56), // 5^-160
    (0xe0f218b8d25088b8, 0x306869c13ec3532c), // 5^-159
    (0x8c974f7383725573, 0x1e414218c73a13fb), // 5^-158
    (0xafbd2350644eeacf, 0xe5d1929ef90898fa), // 5^-157
    (0xdbac6c247d62a583, 0xdf45f746b74abf39), // 5^-156
    (0x894bc396ce5da772, 0x6b8bba8c328eb783), // 5^-155
    (0xab9eb47c81f5114f, 0x66ea92f3f326564),  // 5^-154
    (0xd686619ba27255a2, 0xc80a537b0efefebd), // 5^-153
    (0x8613fd0145877585, 0xbd06742ce95f5f36), // 5^-152
    (0xa798fc4196e952e7, 0x2c48113823b73704), // 5^-151
    (0xd17f3b51fca3a7a0, 0xf75a15862ca504c5), // 5^-150
    (0x82ef85133de648c4, 0x9a984d73dbe722fb), // 5^-149
    (0xa3ab66580d5fdaf5, 0xc13e60d0d2e0ebba), // 5^-148
    (0xcc963fee10b7d1b3, 0x318df905079926a8), // 5^-147
    (0xffbbcfe994e5c61f, 0xfdf17746497f7052), // 5^-146
    (0x9fd561f1fd0f9bd3, 0xfeb6ea8bedefa633), // 5^-145
    (0xc7caba6e7c5382c8, 0xfe64a52ee96b8fc0), // 5^-144
    (0xf9bd690a1b68637b, 0x3dfdce7aa3c673b0), // 5^-143
    (0x9c1661a651213e2d, 0x6bea10ca65c084e),  // 5^-142
    (0xc31bfa0fe5698db8, 0x486e494fcff30a62), // 5^-141
    (0xf3e2f893dec3f126, 0x5a89dba3c3efccfa), // 5^-140
    (0x986ddb5c6b3a76b7, 0xf89629465a75e01c), // 5^-139
    (0xbe89523386091465, 0xf6bbb397f1135823), // 5^-138
    (0xee2ba6c0678b597f, 0x746aa07ded582e2c), // 5^-137
    (0x94db483840b717ef, 0xa8c2a44eb4571cdc), // 5^-136
    (0xba121a4650e4ddeb, 0x92f34d62616ce413), // 5^-135
    (0xe896a0d7e51e1566, 0x77b020baf9c81d17), // 5^-134
    (0x915e2486ef32cd60, 0xace1474dc1d122e),  // 5^-133
    (0xb5b5ada8aaff80b8, 0xd819992132456ba),  // 5^-132
    (0xe3231912d5bf60e6, 0x10e1fff697ed6c69), // 5^-131
    (0x8df5efabc5979c8f, 0xca8d3ffa1ef463c1), // 5^-130
    (0xb1736b96b6fd83b3, 0xbd308ff8a6b17cb2), // 5^-129
    (0xddd0467c64bce4a0, 0xac7cb3f6d05ddbde), // 5^-128
    (0x8aa22c0dbef60ee4, 0x6bcdf07a423aa96b), // 5^-127
    (0xad4ab7112eb3929d, 0x86c16c98d2c953c6), // 5^-126
    (0xd89d64d57a607744, 0xe871c7bf077ba8b7), // 5^-125
    (0x87625f056c7c4a8b, 0x11471cd764ad4972), // 5^-124
    (0xa93af6c6c79b5d2d, 0xd598e40d3dd89bcf), // 5^-123
    (0xd389b47879823479, 0x4aff1d108d4ec2c3), // 5^-122
    (0x843610cb4bf160cb, 0xcedf722a585139ba), // 5^-121
    (0xa54394fe1eedb8fe, 0xc2974eb4ee658828), // 5^-120
    (0xce947a3da6a9273e, 0x733d226229feea32), // 5^-119
    (0x811ccc668829b887, 0x806357d5a3f525f),  // 5^-118
    (0xa163ff802a3426a8, 0xca07c2dcb0cf26f7), // 5^-117
    (0xc9bcff6034c13052, 0xfc89b393dd02f0b5), // 5^-116
    (0xfc2c3f3841f17c67, 0xbbac2078d443ace2), // 5^-115
    (0x9d9ba7832936edc0, 0xd54b944b84aa4c0d), // 5^-114
    (0xc5029163f384a931, 0xa9e795e65d4df11),  // 5^-113
    (0xf64335bcf065d37d, 0x4d4617b5ff4a16d5), // 5^-112
    (0x99ea0196163fa42e, 0x504bced1bf8e4e45), // 5^-111
    (0xc06481fb9bcf8d39, 0xe45ec2862f71e1d6), // 5^-110
    (0xf07da27a82c37088, 0x5d767327bb4e5a4c), // 5^-109
    (0x964e858c91ba2655, 0x3a6a07f8d510f86f), // 5^-108
    (0xbbe226efb628afea, 0x890489f70a55368b), // 5^-107
    (0xeadab0aba3b2dbe5, 0x2b45ac74ccea842e), // 5^-106
    (0x92c8ae6b464fc96f, 0x3b0b8bc90012929d), // 5^-105
    (0xb77ada0617e3bbcb, 0x9ce6ebb40173744),  // 5^-104
    (0xe55990879ddcaabd, 0xcc420a6a101d0515), // 5^-103
    (0x8f57fa54c2a9eab6, 0x9fa946824a12232d), // 5^-102
    (0xb32df8e9f3546564, 0x47939822dc96abf9), // 5^-101
    (0xdff9772470297ebd, 0x59787e2b93bc56f7), // 5^-100
    (0x8bfbea76c619ef36, 0x57eb4edb3c55b65a), // 5^-99
    (0xaefae51477a06b03, 0xede622920b6b23f1), // 5^-98
    (0xdab99e59958885c4, 0xe95fab368e45eced), // 5^-97
    (0x88b402f7fd75539b, 0x11dbcb0218ebb414), // 5^-96
    (0xaae103b5fcd2a881, 0xd652bdc29f26a119), // 5^-95
    (0xd59944a37c0752a2, 0x4be76d3346f0495f), // 5^-94
    (0x857fcae62d8493a5, 0x6f70a4400c562ddb), // 5^-93
    (0xa6dfbd9fb8e5b88e, 0xcb4ccd500f6bb952), // 5^-92
    (0xd097ad07a71f26b2, 0x7e2000a41346a7a7), // 5^-91
    (0x825ecc24c873782f, 0x8ed400668c0c28c8), // 5^-90
    (0xa2f67f2dfa90563b, 0x728900802f0f32fa), // 5^-89
    (0xcbb41ef979346bca, 0x4f2b40a03ad2ffb9), // 5^-88
    (0xfea126b7d78186bc, 0xe2f610c84987bfa8), // 5^-87
    (0x9f24b832e6b0f436, 0xdd9ca7d2df4d7c9),  // 5^-86
    (0xc6ede63fa05d3143, 0x91503d1c79720dbb), // 5^-85
    (0xf8a95fcf88747d94, 0x75a44c6397ce912a), // 5^-84
    (0x9b69dbe1b548ce7c, 0xc986afbe3ee11aba), // 5^-83
    (0xc24452da229b021b, 0xfbe85badce996168), // 5^-82
    (0xf2d56790ab41c2a2, 0xfae27299423fb9c3), // 5^-81
    (0x97c560ba6b0919a5, 0xdccd879fc967d41a), // 5^-80
    (0xbdb6b8e905cb600f, 0x5400e987bbc1c920), // 5^-79
    (0xed246723473e3813, 0x290123e9aab23b68), // 5^-78
    (0x9436c0760c86e30b, 0xf9a0b6720aaf6521), // 5^-77
    (0xb94470938fa89bce, 0xf808e40e8d5b3e69), // 5^-76
    (0xe7958cb87392c2c2, 0xb60b1d1230b20e04), // 5^-75
    (0x90bd77f3483bb9b9, 0xb1c6f22b5e6f48c2), // 5^-74
    (0xb4ecd5f01a4aa828, 0x1e38aeb6360b1af3), // 5^-73
    (0xe2280b6c20dd5232, 0x25c6da63c38de1b0), // 5^-72
    (0x8d590723948a535f, 0x579c487e5a38ad0e), // 5^-71
    (0xb0af48ec79ace837, 0x2d835a9df0c6d851), // 5^-70
    (0xdcdb1b2798182244, 0xf8e431456cf88e65), // 5^-69
    (0x8a08f0f8bf0f156b, 0x1b8e9ecb641b58ff), // 5^-68
    (0xac8b2d36eed2dac5, 0xe272467e3d222f3f), // 5^-67
    (0xd7adf884aa879177, 0x5b0ed81dcc6abb0f), // 5^-66
    (0x86ccbb52ea94baea, 0x98e947129fc2b4e9), // 5^-65
    (0xa87fea27a539e9a5, 0x3f2398d747b36224), // 5^-64
    (0xd29fe4b18e88640e, 0x8eec7f0d19a03aad), // 5^-63
    (0x83a3eeeef9153e89, 0x1953cf68300424ac), // 5^-62
    (0xa48ceaaab75a8e2b, 0x5fa8c3423c052dd7), // 5^-61
    (0xcdb02555653131b6, 0x3792f412cb06794d), // 5^-60
    (0x808e17555f3ebf11, 0xe2bbd88bbee40bd0), // 5^-59
    (0xa0b19d2ab70e6ed6, 0x5b6aceaeae9d0ec4), // 5^-58
    (0xc8de047564d20a8b, 0xf245825a5a445275), // 5^-57
    (0xfb158592be068d2e, 0xeed6e2f0f0d56712), // 5^-56
    (0x9ced737bb6c4183d, 0x55464dd69685606b), // 5^-55
    (0xc428d05aa4751e4c, 0xaa97e14c3c26b886), // 5^-54
    (0xf53304714d9265df, 0xd53dd99f4b3066a8), // 5^-53
    (0x993fe2c6d07b7fab, 0xe546a8038efe4029), // 5^-52
    (0xbf8fdb78849a5f96, 0xde98520472bdd033), // 5^-51
    (0xef73d256a5c0f77c, 0x963e66858f6d4440), // 5^-50
    (0x95a8637627989aad, 0xdde7001379a44aa8), // 5^-49
    (0xbb127c53b17ec159, 0x5560c018580d5d52), // 5^-48
    (0xe9d71b689dde71af, 0xaab8f01e6e10b4a6), // 5^-47
    (0x9226712162ab070d, 0xcab3961304ca70e8), // 5^-46
    (0xb6b00d69bb55c8d1, 0x3d607b97c5fd0d22), // 5^-45
    (0xe45c10c42a2b3b05, 0x8cb89a7db77c506a), // 5^-44
    (0x8eb98a7a9a5b04e3, 0x77f3608e92adb242), // 5^-43
    (0xb267ed1940f1c61c, 0x55f038b237591ed3), // 5^-42
    (0xdf01e85f912e37a3, 0x6b6c46dec52f6688), // 5^-41
    (0x8b61313bbabce2c6, 0x2323ac4b3b3da015), // 5^-40
    (0xae397d8aa96c1b77, 0xabec975e0a0d081a), // 5^-39
    (0xd9c7dced53c72255, 0x96e7bd358c904a21), // 5^-38
    (0x881cea14545c7575, 0x7e50d64177da2e54), // 5^-37
    (0xaa242499697392d2, 0xdde50bd1d5d0b9e9), // 5^-36
    (0xd4ad2dbfc3d07787, 0x955e4ec64b44e864), // 5^-35
    (0x84ec3c97da624ab4, 0xbd5af13bef0b113e), // 5^-34
    (0xa6274bbdd0fadd61, 0xecb1ad8aeacdd58e), // 5^-33
    (0xcfb11ead453994ba, 0x67de18eda5814af2), // 5^-32
    (0x81ceb32c4b43fcf4, 0x80eacf948770ced7), // 5^-31
    (0xa2425ff75e14fc31, 0xa1258379a94d028d), // 5^-30
    (0xcad2f7f5359a3b3e, 0x96ee45813a04330),  // 5^-29
    (0xfd87b5f28300ca0d, 0x8bca9d6e188853fc), // 5^-28
    (0x9e74d1b791e07e48, 0x775ea264cf55347e), // 5^-27
    (0xc612062576589dda, 0x95364afe032a819e), // 5^-26
    (0xf79687aed3eec551, 0x3a83ddbd83f52205), // 5^-25
    (0x9abe14cd44753b52, 0xc4926a9672793543), // 5^-24
    (0xc16d9a0095928a27, 0x75b7053c0f178294), // 5^-23
    (0xf1c90080baf72cb1, 0x5324c68b12dd6339), // 5^-22
    (0x971da05074da7bee, 0xd3f6fc16ebca5e04), // 5^-21
    (0xbce5086492111aea, 0x88f4bb1ca6bcf585), // 5^-20
    (0xec1e4a7db69561a5, 0x2b31e9e3d06c32e6), // 5^-19
    (0x9392ee8e921d5d07, 0x3aff322e62439fd0), // 5^-18
    (0xb877aa3236a4b449, 0x9befeb9fad487c3),  // 5^-17
    (0xe69594bec44de15b, 0x4c2ebe687989a9b4), // 5^-16
    (0x901d7cf73ab0acd9, 0xf9d37014bf60a11),  // 5^-15
    (0xb424dc35095cd80f, 0x538484c19ef38c95), // 5^-14
    (0xe12e13424bb40e13, 0x2865a5f206b06fba), // 5^-13
    (0x8cbccc096f5088cb, 0xf93f87b7442e45d4), // 5^-12
    (0xafebff0bcb24aafe, 0xf78f69a51539d749), // 5^-11
    (0xdbe6fecebdedd5be, 0xb573440e5a884d1c), // 5^-10
    (0x89705f4136b4a597, 0x31680a88f8953031), // 5^-9
    (0xabcc77118461cefc, 0xfdc20d2b36ba7c3e), // 5^-8
    (0xd6bf94d5e57a42bc, 0x3d32907604691b4d), // 5^-7
    (0x8637bd05af6c69b5, 0xa63f9a49c2c1b110), // 5^-6
    (0xa7c5ac471b478423, 0xfcf80dc33721d54),  // 5^-5
    (0xd1b71758e219652b, 0xd3c36113404ea4a9), // 5^-4
    (0x83126e978d4fdf3b, 0x645a1cac083126ea), // 5^-3
    (0xa3d70a3d70a3d70a, 0x3d70a3d70a3d70a4), // 5^-2
    (0xcccccccccccccccc, 0xcccccccccccccccd), // 5^-1
    (0x8000000000000000, 0x0),                // 5^0
    (0xa000000000000000, 0x0),                // 5^1
    (0xc800000000000000, 0x0),                // 5^2
    (0xfa00000000000000, 0x0),                // 5^3
    (0x9c40000000000000, 0x0),                // 5^4
    (0xc350000000000000, 0x0),                // 5^5
    (0xf424000000000000, 0x0),                // 5^6
    (0x9896800000000000, 0x0),                // 5^7
    (0xbebc200000000000, 0x0),                // 5^8
    (0xee6b280000000000, 0x0),                // 5^9
    (0x9502f90000000000, 0x0),                // 5^10
    (0xba43b74000000000, 0x0),                // 5^11
    (0xe8d4a51000000000, 0x0),                // 5^12
    (0x9184e72a00000000, 0x0),                // 5^13
    (0xb5e620f480000000, 0x0),                // 5^14
    (0xe35fa931a0000000, 0x0),                // 5^15
    (0x8e1bc9bf04000000, 0x0),                // 5^16
    (0xb1a2bc2ec5000000, 0x0),                // 5^17
    (0xde0b6b3a76400000, 0x0),                // 5^18
    (0x8ac7230489e80000, 0x0),                // 5^19
    (0xad78ebc5ac620000, 0x0),                // 5^20
    (0xd8d726b7177a8000, 0x0),                // 5^21
    (0x878678326eac9000, 0x0),                // 5^22
    (0xa968163f0a57b400, 0x0),                // 5^23
    (0xd3c21bcecceda100, 0x0),                // 5^24
    (0x84595161401484a0, 0x0),                // 5^25
    (0xa56fa5b99019a5c8, 0x0),                // 5^26
    (0xcecb8f27f4200f3a, 0x0),                // 5^27
    (0x813f3978f8940984, 0x4000000000000000), // 5^28
    (0xa18f07d736b90be5, 0x5000000000000000), // 5^29
    (0xc9f2c9cd04674ede, 0xa400000000000000), // 5^30
    (0xfc6f7c4045812296, 0x4d00000000000000), // 5^31
    (0x9dc5ada82b70b59d, 0xf020000000000000), // 5^32
    (0xc5371912364ce305, 0x6c28000000000000), // 5^33
    (0xf684df56c3e01bc6, 0xc732000000000000), // 5^34
    (0x9a130b963a6c115c, 0x3c7f400000000000), // 5^35
    (0xc097ce7bc90715b3, 0x4b9f100000000000), // 5^36
    (0xf0bdc21abb48db20, 0x1e86d40000000000), // 5^37
    (0x96769950b50d88f4, 0x1314448000000000), // 5^38
    (0xbc143fa4e250eb31, 0x17d955a000000000), // 5^39
    (0xeb194f8e1ae525fd, 0x5dcfab0800000000), // 5^40
    (0x92efd1b8d0cf37be, 0x5aa1cae500000000), // 5^41
    (0xb7abc627050305ad, 0xf14a3d9e40000000), // 5^42
    (0xe596b7b0c643c719, 0x6d9ccd05d0000000), // 5^43
    (0x8f7e32ce7bea5c6f, 0xe4820023a2000000), // 5^44
    (0xb35dbf821ae4f38b, 0xdda2802c8a800000), // 5^45
    (0xe0352f62a19e306e, 0xd50b2037ad200000), // 5^46
    (0x8c213d9da502de45, 0x4526f422cc340000), // 5^47
    (0xaf298d050e4395d6, 0x9670b12b7f410000), // 5^48
    (0xdaf3f04651d47b4c, 0x3c0cdd765f114000), // 5^49
    (0x88d8762bf324cd0f, 0xa5880a69fb6ac800), // 5^50
    (0xab0e93b6efee0053, 0x8eea0d047a457a00), // 5^51
    (0xd5d238a4abe98068, 0x72a4904598d6d880), // 5^52
    (0x85a36366eb71f041, 0x47a6da2b7f864750), // 5^53
    (0xa70c3c40a64e6c51, 0x999090b65f67d924), // 5^54
    (0xd0cf4b50cfe20765, 0xfff4b4e3f741cf6d), // 5^55
    (0x82818f1281ed449f, 0xbff8f10e7a8921a4), // 5^56
    (0xa321f2d7226895c7, 0xaff72d52192b6a0d), // 5^57
    (0xcbea6f8ceb02bb39, 0x9bf4f8a69f764490), // 5^58
    (0xfee50b7025c36a08, 0x2f236d04753d5b4),  // 5^59
    (0x9f4f2726179a2245, 0x1d762422c946590),  // 5^60
    (0xc722f0ef9d80aad6, 0x424d3ad2b7b97ef5), // 5^61
    (0xf8ebad2b84e0d58b, 0xd2e0898765a7deb2), // 5^62
    (0x9b934c3b330c8577, 0x63cc55f49f88eb2f), // 5^63
    (0xc2781f49ffcfa6d5, 0x3cbf6b71c76b25fb), // 5^64
    (0xf316271c7fc3908a, 0x8bef464e3945ef7a), // 5^65
    (0x97edd871cfda3a56, 0x97758bf0e3cbb5ac), // 5^66
    (0xbde94e8e43d0c8ec, 0x3d52eeed1cbea317), // 5^67
    (0xed63a231d4c4fb27, 0x4ca7aaa863ee4bdd), // 5^68
    (0x945e455f24fb1cf8, 0x8fe8caa93e74ef6a), // 5^69
    (0xb975d6b6ee39e436, 0xb3e2fd538e122b44), // 5^70
    (0xe7d34c64a9c85d44, 0x60dbbca87196b616), // 5^71
    (0x90e40fbeea1d3a4a, 0xbc8955e946fe31cd), // 5^72
    (0xb51d13aea4a488dd, 0x6babab6398bdbe41), // 5^73
    (0xe264589a4dcdab14, 0xc696963c7eed2dd1), // 5^74
    (0x8d7eb76070a08aec, 0xfc1e1de5cf543ca2), // 5^75
    (0xb0de65388cc8ada8, 0x3b25a55f43294bcb), // 5^76
    (0xdd15fe86affad912, 0x49ef0eb713f39ebe), // 5^77
    (0x8a2dbf142dfcc7ab, 0x6e3569326c784337), // 5^78
    (0xacb92ed9397bf996, 0x49c2c37f07965404), // 5^79
    (0xd7e77a8f87daf7fb, 0xdc33745ec97be906), // 5^80
    (0x86f0ac99b4e8dafd, 0x69a028bb3ded71a3), // 5^81
    (0xa8acd7c0222311bc, 0xc40832ea0d68ce0c), // 5^82
    (0xd2d80db02aabd62b, 0xf50a3fa490c30190), // 5^83
    (0x83c7088e1aab65db, 0x792667c6da79e0fa), // 5^84
    (0xa4b8cab1a1563f52, 0x577001b891185938), // 5^85
    (0xcde6fd5e09abcf26, 0xed4c0226b55e6f86), // 5^86
    (0x80b05e5ac60b6178, 0x544f8158315b05b4), // 5^87
    (0xa0dc75f1778e39d6, 0x696361ae3db1c721), // 5^88
    (0xc913936dd571c84c, 0x3bc3a19cd1e38e9),  // 5^89
    (0xfb5878494ace3a5f, 0x4ab48a04065c723),  // 5^90
    (0x9d174b2dcec0e47b, 0x62eb0d64283f9c76), // 5^91
    (0xc45d1df942711d9a, 0x3ba5d0bd324f8394), // 5^92
    (0xf5746577930d6500, 0xca8f44ec7ee36479), // 5^93
    (0x9968bf6abbe85f20, 0x7e998b13cf4e1ecb), // 5^94
    (0xbfc2ef456ae276e8, 0x9e3fedd8c321a67e), // 5^95
    (0xefb3ab16c59b14a2, 0xc5cfe94ef3ea101e), // 5^96
    (0x95d04aee3b80ece5, 0xbba1f1d158724a12), // 5^97
    (0xbb445da9ca61281f, 0x2a8a6e45ae8edc97), // 5^98
    (0xea1575143cf97226, 0xf52d09d71a3293bd), // 5^99
    (0x924d692ca61be758, 0x593c2626705f9c56), // 5^100
    (0xb6e0c377cfa2e12e, 0x6f8b2fb00c77836c), // 5^101
    (0xe498f455c38b997a, 0xb6dfb9c0f956447),  // 5^102
    (0x8edf98b59a373fec, 0x4724bd4189bd5eac), // 5^103
    (0xb2977ee300c50fe7, 0x58edec91ec2cb657), // 5^104
    (0xdf3d5e9bc0f653e1, 0x2f2967b66737e3ed), // 5^105
    (0x8b865b215899f46c, 0xbd79e0d20082ee74), // 5^106
    (0xae67f1e9aec07187, 0xecd8590680a3aa11), // 5^107
    (0xda01ee641a708de9, 0xe80e6f4820cc9495), // 5^108
    (0x884134fe908658b2, 0x3109058d147fdcdd), // 5^109
    (0xaa51823e34a7eede, 0xbd4b46f0599fd415), // 5^110
    (0xd4e5e2cdc1d1ea96, 0x6c9e18ac7007c91a), // 5^111
    (0x850fadc09923329e, 0x3e2cf6bc604ddb0),  // 5^112
    (0xa6539930bf6bff45, 0x84db8346b786151c), // 5^113
    (0xcfe87f7cef46ff16, 0xe612641865679a63), // 5^114
    (0x81f14fae158c5f6e, 0x4fcb7e8f3f60c07e), // 5^115
    (0xa26da3999aef7749, 0xe3be5e330f38f09d), // 5^116
    (0xcb090c8001ab551c, 0x5cadf5bfd3072cc5), // 5^117
    (0xfdcb4fa002162a63, 0x73d9732fc7c8f7f6), // 5^118
    (0x9e9f11c4014dda7e, 0x2867e7fddcdd9afa), // 5^119
    (0xc646d63501a1511d, 0xb281e1fd541501b8), // 5^120
    (0xf7d88bc24209a565, 0x1f225a7ca91a4226), // 5^121
    (0x9ae757596946075f, 0x3375788de9b06958), // 5^122
    (0xc1a12d2fc3978937, 0x52d6b1641c83ae),   // 5^123
    (0xf209787bb47d6b84, 0xc0678c5dbd23a49a), // 5^124
    (0x9745eb4d50ce6332, 0xf840b7ba963646e0), // 5^125
    (0xbd176620a501fbff, 0xb650e5a93bc3d898), // 5^126
    (0xec5d3fa8ce427aff, 0xa3e51f138ab4cebe), // 5^127
    (0x93ba47c980e98cdf, 0xc66f336c36b10137), // 5^128
    (0xb8a8d9bbe123f017, 0xb80b0047445d4184), // 5^129
    (0xe6d3102ad96cec1d, 0xa60dc059157491e5), // 5^130
    (0x9043ea1ac7e41392, 0x87c89837ad68db2f), // 5^131
    (0xb454e4a179dd1877, 0x29babe4598c311fb), // 5^132
    (0xe16a1dc9d8545e94, 0xf4296dd6fef3d67a), // 5^133
    (0x8ce2529e2734bb1d, 0x1899e4a65f58660c), // 5^134
    (0xb01ae745b101e9e4, 0x5ec05dcff72e7f8f), // 5^135
    (0xdc21a1171d42645d, 0x76707543f4fa1f73), // 5^136
    (0x899504ae72497eba, 0x6a06494a791c53a8), // 5^137
    (0xabfa45da0edbde69, 0x487db9d17636892),  // 5^138
    (0xd6f8d7509292d603, 0x45a9d2845d3c42b6), // 5^139
    (0x865b86925b9bc5c2, 0xb8a2392ba45a9b2),  // 5^140
    (0xa7f26836f282b732, 0x8e6cac7768d7141e), // 5^141
    (0xd1ef0244af2364ff, 0x3207d795430cd926), // 5^142
    (0x8335616aed761f1f, 0x7f44e6bd49e807b8), // 5^143
    (0xa402b9c5a8d3a6e7, 0x5f16206c9c6209a6), // 5^144
    (0xcd036837130890a1, 0x36dba887c37a8c0f), // 5^145
    (0x802221226be55a64, 0xc2494954da2c9789), // 5^146
    (0xa02aa96b06deb0fd, 0xf2db9baa10b7bd6c), // 5^147
    (0xc83553c5c8965d3d, 0x6f92829494e5acc7), // 5^148
    (0xfa42a8b73abbf48c, 0xcb772339ba1f17f9), // 5^149
    (0x9c69a97284b578d7, 0xff2a760414536efb), // 5^150
    (0xc38413cf25e2d70d, 0xfef5138519684aba), // 5^151
    (0xf46518c2ef5b8cd1, 0x7eb258665fc25d69), // 5^152
    (0x98bf2f79d5993802, 0xef2f773ffbd97a61), // 5^153
    (0xbeeefb584aff8603, 0xaafb550ffacfd8fa), // 5^154
    (0xeeaaba2e5dbf6784, 0x95ba2a53f983cf38), // 5^155
    (0x952ab45cfa97a0b2, 0xdd945a747bf26183), // 5^156
    (0xba756174393d88df, 0x94f971119aeef9e4), // 5^157
    (0xe912b9d1478ceb17, 0x7a37cd5601aab85d), // 5^158
    (0x91abb422ccb812ee, 0xac62e055c10ab33a), // 5^159
    (0xb616a12b7fe617aa, 0x577b986b314d6009), // 5^160
    (0xe39c49765fdf9d94, 0xed5a7e85fda0b80b), // 5^161
    (0x8e41ade9fbebc27d, 0x14588f13be847307), // 5^162
    (0xb1d219647ae6b31c, 0x596eb2d8ae258fc8), // 5^163
    (0xde469fbd99a05fe3, 0x6fca5f8ed9aef3bb), // 5^164
    (0x8aec23d680043bee, 0x25de7bb9480d5854), // 5^165
    (0xada72ccc20054ae9, 0xaf561aa79a10ae6a), // 5^166
    (0xd910f7ff28069da4, 0x1b2ba1518094da04), // 5^167
    (0x87aa9aff79042286, 0x90fb44d2f05d0842), // 5^168
    (0xa99541bf57452b28, 0x353a1607ac744a53), // 5^169
    (0xd3fa922f2d1675f2, 0x42889b8997915ce8), // 5^170
    (0x847c9b5d7c2e09b7, 0x69956135febada11), // 5^171
    (0xa59bc234db398c25, 0x43fab9837e699095), // 5^172
    (0xcf02b2c21207ef2e, 0x94f967e45e03f4bb), // 5^173
    (0x8161afb94b44f57d, 0x1d1be0eebac278f5), // 5^174
    (0xa1ba1ba79e1632dc, 0x6462d92a69731732), // 5^175
    (0xca28a291859bbf93, 0x7d7b8f7503cfdcfe), // 5^176
    (0xfcb2cb35e702af78, 0x5cda735244c3d43e), // 5^177
    (0x9defbf01b061adab, 0x3a0888136afa64a7), // 5^178
    (0xc56baec21c7a1916, 0x88aaa1845b8fdd0),  // 5^179
    (0xf6c69a72a3989f5b, 0x8aad549e57273d45), // 5^180
    (0x9a3c2087a63f6399, 0x36ac54e2f678864b), // 5^181
    (0xc0cb28a98fcf3c7f, 0x84576a1bb416a7dd), // 5^182
    (0xf0fdf2d3f3c30b9f, 0x656d44a2a11c51d5), // 5^183
    (0x969eb7c47859e743, 0x9f644ae5a4b1b325), // 5^184
    (0xbc4665b596706114, 0x873d5d9f0dde1fee), // 5^185
    (0xeb57ff22fc0c7959, 0xa90cb506d155a7ea), // 5^186
    (0x9316ff75dd87cbd8, 0x9a7f12442d588f2),  // 5^187
    (0xb7dcbf5354e9bece, 0xc11ed6d538aeb2f),  // 5^188
    (0xe5d3ef282a242e81, 0x8f1668c8a86da5fa), // 5^189
    (0x8fa475791a569d10, 0xf96e017d694487bc), // 5^190
    (0xb38d92d760ec4455, 0x37c981dcc395a9ac), // 5^191
    (0xe070f78d3927556a, 0x85bbe253f47b1417), // 5^192
    (0x8c469ab843b89562, 0x93956d7478ccec8e), // 5^193
    (0xaf58416654a6babb, 0x387ac8d1970027b2), // 5^194
    (0xdb2e51bfe9d0696a, 0x6997b05fcc0319e),  // 5^195
    (0x88fcf317f22241e2, 0x441fece3bdf81f03), // 5^196
    (0xab3c2fddeeaad25a, 0xd527e81cad7626c3), // 5^197
    (0xd60b3bd56a5586f1, 0x8a71e223d8d3b074), // 5^198
    (0x85c7056562757456, 0xf6872d5667844e49), // 5^199
    (0xa738c6bebb12d16c, 0xb428f8ac016561db), // 5^200
    (0xd106f86e69d785c7, 0xe13336d701beba52), // 5^201
    (0x82a45b450226b39c, 0xecc0024661173473), // 5^202
    (0xa34d721642b06084, 0x27f002d7f95d0190), // 5^203
    (0xcc20ce9bd35c78a5, 0x31ec038df7b441f4), // 5^204
    (0xff290242c83396ce, 0x7e67047175a15271), // 5^205
    (0x9f79a169bd203e41, 0xf0062c6e984d386),  // 5^206
    (0xc75809c42c684dd1, 0x52c07b78a3e60868), // 5^207
    (0xf92e0c3537826145, 0xa7709a56ccdf8a82), // 5^208
    (0x9bbcc7a142b17ccb, 0x88a66076400bb691), // 5^209
    (0xc2abf989935ddbfe, 0x6acff893d00ea435), // 5^210
    (0xf356f7ebf83552fe, 0x583f6b8c4124d43),  // 5^211
    (0x98165af37b2153de, 0xc3727a337a8b704a), // 5^212
    (0xbe1bf1b059e9a8d6, 0x744f18c0592e4c5c), // 5^213
    (0xeda2ee1c7064130c, 0x1162def06f79df73), // 5^214
    (0x9485d4d1c63e8be7, 0x8addcb5645ac2ba8), // 5^215
    (0xb9a74a0637ce2ee1, 0x6d953e2bd7173692), // 5^216
    (0xe8111c87c5c1ba99, 0xc8fa8db6ccdd0437), // 5^217
    (0x910ab1d4db9914a0, 0x1d9c9892400a22a2), // 5^218
    (0xb54d5e4a127f59c8, 0x2503beb6d00cab4b), // 5^219
    (0xe2a0b5dc971f303a, 0x2e44ae64840fd61d), // 5^220
    (0x8da471a9de737e24, 0x5ceaecfed289e5d2), // 5^221
    (0xb10d8e1456105dad, 0x7425a83e872c5f47), // 5^222
    (0xdd50f1996b947518, 0xd12f124e28f77719), // 5^223
    (0x8a5296ffe33cc92f, 0x82bd6b70d99aaa6f), // 5^224
    (0xace73cbfdc0bfb7b, 0x636cc64d1001550b), // 5^225
    (0xd8210befd30efa5a, 0x3c47f7e05401aa4e), // 5^226
    (0x8714a775e3e95c78, 0x65acfaec34810a71), // 5^227
    (0xa8d9d1535ce3b396, 0x7f1839a741a14d0d), // 5^228
    (0xd31045a8341ca07c, 0x1ede48111209a050), // 5^229
    (0x83ea2b892091e44d, 0x934aed0aab460432), // 5^230
    (0xa4e4b66b68b65d60, 0xf81da84d5617853f), // 5^231
    (0xce1de40642e3f4b9, 0x36251260ab9d668e), // 5^232
    (0x80d2ae83e9ce78f3, 0xc1d72b7c6b426019), // 5^233
    (0xa1075a24e4421730, 0xb24cf65b8612f81f), // 5^234
    (0xc94930ae1d529cfc, 0xdee033f26797b627), // 5^235
    (0xfb9b7cd9a4a7443c, 0x169840ef017da3b1), // 5^236
    (0x9d412e0806e88aa5, 0x8e1f289560ee864e), // 5^237
    (0xc491798a08a2ad4e, 0xf1a6f2bab92a27e2), // 5^238
    (0xf5b5d7ec8acb58a2, 0xae10af696774b1db), // 5^239
    (0x9991a6f3d6bf1765, 0xacca6da1e0a8ef29), // 5^240
    (0xbff610b0cc6edd3f, 0x17fd090a58d32af3), // 5^241
    (0xeff394dcff8a948e, 0xddfc4b4cef07f5b0), // 5^242
    (0x95f83d0a1fb69cd9, 0x4abdaf101564f98e), // 5^243
    (0xbb764c4ca7a4440f, 0x9d6d1ad41abe37f1), // 5^244
    (0xea53df5fd18d5513, 0x84c86189216dc5ed), // 5^245
    (0x92746b9be2f8552c, 0x32fd3cf5b4e49bb4), // 5^246
    (0xb7118682dbb66a77, 0x3fbc8c33221dc2a1), // 5^247
    (0xe4d5e82392a40515, 0xfabaf3feaa5334a),  // 5^248
    (0x8f05b1163ba6832d, 0x29cb4d87f2a7400e), // 5^249
    (0xb2c71d5bca9023f8, 0x743e20e9ef511012), // 5^250
    (0xdf78e4b2bd342cf6, 0x914da9246b255416), // 5^251
    (0x8bab8eefb6409c1a, 0x1ad089b6c2f7548e), // 5^252
    (0xae9672aba3d0c320, 0xa184ac2473b529b1), // 5^253
    (0xda3c0f568cc4f3e8, 0xc9e5d72d90a2741e), // 5^254
    (0x8865899617fb1871, 0x7e2fa67c7a658892), // 5^255
    (0xaa7eebfb9df9de8d, 0xddbb901b98feeab7), // 5^256
    (0xd51ea6fa85785631, 0x552a74227f3ea565), // 5^257
    (0x8533285c936b35de, 0xd53a88958f87275f), // 5^258
    (0xa67ff273b8460356, 0x8a892abaf368f137), // 5^259
    (0xd01fef10a657842c, 0x2d2b7569b0432d85), // 5^260
    (0x8213f56a67f6b29b, 0x9c3b29620e29fc73), // 5^261
    (0xa298f2c501f45f42, 0x8349f3ba91b47b8f), // 5^262
    (0xcb3f2f7642717713, 0x241c70a936219a73), // 5^263
    (0xfe0efb53d30dd4d7, 0xed238cd383aa0110), // 5^264
    (0x9ec95d1463e8a506, 0xf4363804324a40aa), // 5^265
    (0xc67bb4597ce2ce48, 0xb143c6053edcd0d5), // 5^266
    (0xf81aa16fdc1b81da, 0xdd94b7868e94050a), // 5^267
    (0x9b10a4e5e9913128, 0xca7cf2b4191c8326), // 5^268
    (0xc1d4ce1f63f57d72, 0xfd1c2f611f63a3f0), // 5^269
    (0xf24a01a73cf2dccf, 0xbc633b39673c8cec), // 5^270
    (0x976e41088617ca01, 0xd5be0503e085d813), // 5^271
    (0xbd49d14aa79dbc82, 0x4b2d8644d8a74e18), // 5^272
    (0xec9c459d51852ba2, 0xddf8e7d60ed1219e), // 5^273
    (0x93e1ab8252f33b45, 0xcabb90e5c942b503), // 5^274
    (0xb8da1662e7b00a17, 0x3d6a751f3b936243), // 5^275
    (0xe7109bfba19c0c9d, 0xcc512670a783ad4),  // 5^276
    (0x906a617d450187e2, 0x27fb2b80668b24c5), // 5^277
    (0xb484f9dc9641e9da, 0xb1f9f660802dedf6), // 5^278
    (0xe1a63853bbd26451, 0x5e7873f8a0396973), // 5^279
    (0x8d07e33455637eb2, 0xdb0b487b6423e1e8), // 5^280
    (0xb049dc016abc5e5f, 0x91ce1a9a3d2cda62), // 5^281
    (0xdc5c5301c56b75f7, 0x7641a140cc7810fb), // 5^282
    (0x89b9b3e11b6329ba, 0xa9e904c87fcb0a9d), // 5^283
    (0xac2820d9623bf429, 0x546345fa9fbdcd44), // 5^284
    (0xd732290fbacaf133, 0xa97c177947ad4095), // 5^285
    (0x867f59a9d4bed6c0, 0x49ed8eabcccc485d), // 5^286
    (0xa81f301449ee8c70, 0x5c68f256bfff5a74), // 5^287
    (0xd226fc195c6a2f8c, 0x73832eec6fff3111), // 5^288
    (0x83585d8fd9c25db7, 0xc831fd53c5ff7eab), // 5^289
    (0xa42e74f3d032f525, 0xba3e7ca8b77f5e55), // 5^290
    (0xcd3a1230c43fb26f, 0x28ce1bd2e55f35eb), // 5^291
    (0x80444b5e7aa7cf85, 0x7980d163cf5b81b3), // 5^292
    (0xa0555e361951c366, 0xd7e105bcc332621f), // 5^293
    (0xc86ab5c39fa63440, 0x8dd9472bf3fefaa7), // 5^294
    (0xfa856334878fc150, 0xb14f98f6f0feb951), // 5^295
    (0x9c935e00d4b9d8d2, 0x6ed1bf9a569f33d3), // 5^296
    (0xc3b8358109e84f07, 0xa862f80ec4700c8),  // 5^297
    (0xf4a642e14c6262c8, 0xcd27bb612758c0fa), // 5^298
    (0x98e7e9cccfbd7dbd, 0x8038d51cb897789c), // 5^299
    (0xbf21e44003acdd2c, 0xe0470a63e6bd56c3), // 5^300
    (0xeeea5d5004981478, 0x1858ccfce06cac74), // 5^301
    (0x95527a5202df0ccb, 0xf37801e0c43ebc8),  // 5^302
    (0xbaa718e68396cffd, 0xd30560258f54e6ba), // 5^303
    (0xe950df20247c83fd, 0x47c6b82ef32a2069), // 5^304
    (0x91d28b7416cdd27e, 0x4cdc331d57fa5441), // 5^305
    (0xb6472e511c81471d, 0xe0133fe4adf8e952), // 5^306
    (0xe3d8f9e563a198e5, 0x58180fddd97723a6), // 5^307
    (0x8e679c2f5e44ff8f, 0x570f09eaa7ea7648), // 5^308
];
