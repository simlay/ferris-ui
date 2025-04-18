(function() {
    var type_impls = Object.fromEntries([["objc2_core_foundation",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#impl-Clone-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#impl-Debug-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#46-68\">Source</a><a href=\"#impl-Encode-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"objc2/encode/trait.Encode.html\" title=\"trait objc2::encode::Encode\">Encode</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.ENCODING\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#47-67\">Source</a><a href=\"#associatedconstant.ENCODING\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"objc2/encode/trait.Encode.html#associatedconstant.ENCODING\" class=\"constant\">ENCODING</a>: <a class=\"enum\" href=\"objc2_encode/encoding/enum.Encoding.html\" title=\"enum objc2_encode::encoding::Encoding\">Encoding</a></h4></section></summary><div class='docblock'>The Objective-C type-encoding for this type.</div></details></div></details>","Encode","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%5Bu8;+16%5D%3E-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/uuid.rs.html#3-25\">Source</a><a href=\"#impl-From%3C%5Bu8;+16%5D%3E-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">16</a>]&gt; for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/uuid.rs.html#5-24\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(value: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">16</a>]) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<[u8; 16]>","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#impl-PartialEq-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#262\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RefEncode-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#71-73\">Source</a><a href=\"#impl-RefEncode-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"objc2/encode/trait.RefEncode.html\" title=\"trait objc2::encode::RefEncode\">RefEncode</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.ENCODING_REF\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#72\">Source</a><a href=\"#associatedconstant.ENCODING_REF\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"objc2/encode/trait.RefEncode.html#associatedconstant.ENCODING_REF\" class=\"constant\">ENCODING_REF</a>: <a class=\"enum\" href=\"objc2_encode/encoding/enum.Encoding.html\" title=\"enum objc2_encode::encoding::Encoding\">Encoding</a></h4></section></summary><div class='docblock'>The Objective-C type-encoding for a reference of this type. <a href=\"objc2/encode/trait.RefEncode.html#associatedconstant.ENCODING_REF\">Read more</a></div></details></div></details>","RefEncode","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<section id=\"impl-Copy-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#impl-Copy-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section>","Copy","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"],["<section id=\"impl-StructuralPartialEq-for-CFUUIDBytes\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2_core_foundation/generated/CFUUID.rs.html#25\">Source</a><a href=\"#impl-StructuralPartialEq-for-CFUUIDBytes\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"objc2_core_foundation/struct.CFUUIDBytes.html\" title=\"struct objc2_core_foundation::CFUUIDBytes\">CFUUIDBytes</a></h3></section>","StructuralPartialEq","objc2_core_foundation::generated::__CFPlugInCOM::REFIID"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[12320]}