(function() {
    var type_impls = Object.fromEntries([["objc2",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ProtocolBuilder\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#463\">Source</a><a href=\"#impl-Debug-for-ProtocolBuilder\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\">ProtocolBuilder</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#463\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","objc2::declare::ProtocolDecl"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-ProtocolBuilder\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#563-570\">Source</a><a href=\"#impl-Drop-for-ProtocolBuilder\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\">ProtocolBuilder</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#565-569\">Source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","objc2::declare::ProtocolDecl"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ProtocolBuilder\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#472-561\">Source</a><a href=\"#impl-ProtocolBuilder\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\">ProtocolBuilder</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#486-489\">Source</a><h4 class=\"code-header\">pub fn <a href=\"objc2/runtime/struct.ProtocolBuilder.html#tymethod.new\" class=\"fn\">new</a>(name: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html\" title=\"struct core::ffi::c_str::CStr\">CStr</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Self&gt;</h4></section></summary><div class=\"docblock\"><p>Constructs a <a href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\"><code>ProtocolBuilder</code></a> with the given name.</p>\n<p>Returns <a href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None\" title=\"variant core::option::Option::None\"><code>None</code></a> if the protocol couldn’t be allocated.</p>\n<h5 id=\"panics\"><a class=\"doc-anchor\" href=\"#panics\">§</a>Panics</h5>\n<p>Panics if the name contains an internal NULL byte.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_method_description\" class=\"method\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#519-531\">Source</a><h4 class=\"code-header\">pub fn <a href=\"objc2/runtime/struct.ProtocolBuilder.html#tymethod.add_method_description\" class=\"fn\">add_method_description</a>&lt;Args, Ret&gt;(&amp;mut self, sel: <a class=\"struct\" href=\"objc2/runtime/struct.Sel.html\" title=\"struct objc2::runtime::Sel\">Sel</a>, required: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>)<div class=\"where\">where\n    Args: <a class=\"trait\" href=\"objc2/encode/trait.EncodeArguments.html\" title=\"trait objc2::encode::EncodeArguments\">EncodeArguments</a>,\n    Ret: <a class=\"trait\" href=\"objc2/encode/trait.EncodeReturn.html\" title=\"trait objc2::encode::EncodeReturn\">EncodeReturn</a>,</div></h4></section></summary><div class=\"docblock\"><p>Add an instance method with a given description.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_class_method_description\" class=\"method\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#534-546\">Source</a><h4 class=\"code-header\">pub fn <a href=\"objc2/runtime/struct.ProtocolBuilder.html#tymethod.add_class_method_description\" class=\"fn\">add_class_method_description</a>&lt;Args, Ret&gt;(\n    &amp;mut self,\n    sel: <a class=\"struct\" href=\"objc2/runtime/struct.Sel.html\" title=\"struct objc2::runtime::Sel\">Sel</a>,\n    required: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>,\n)<div class=\"where\">where\n    Args: <a class=\"trait\" href=\"objc2/encode/trait.EncodeArguments.html\" title=\"trait objc2::encode::EncodeArguments\">EncodeArguments</a>,\n    Ret: <a class=\"trait\" href=\"objc2/encode/trait.EncodeReturn.html\" title=\"trait objc2::encode::EncodeReturn\">EncodeReturn</a>,</div></h4></section></summary><div class=\"docblock\"><p>Add a class method with a given description.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.add_protocol\" class=\"method\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#549-551\">Source</a><h4 class=\"code-header\">pub fn <a href=\"objc2/runtime/struct.ProtocolBuilder.html#tymethod.add_protocol\" class=\"fn\">add_protocol</a>(&amp;mut self, proto: &amp;<a class=\"struct\" href=\"objc2/runtime/struct.AnyProtocol.html\" title=\"struct objc2::runtime::AnyProtocol\">AnyProtocol</a>)</h4></section></summary><div class=\"docblock\"><p>Adds a requirement on another protocol.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.register\" class=\"method\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#555-560\">Source</a><h4 class=\"code-header\">pub fn <a href=\"objc2/runtime/struct.ProtocolBuilder.html#tymethod.register\" class=\"fn\">register</a>(self) -&gt; &amp;'static <a class=\"struct\" href=\"objc2/runtime/struct.AnyProtocol.html\" title=\"struct objc2::runtime::AnyProtocol\">AnyProtocol</a></h4></section></summary><div class=\"docblock\"><p>Registers the <a href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\"><code>ProtocolBuilder</code></a>, consuming it and returning a reference\nto the newly registered <a href=\"objc2/runtime/struct.AnyProtocol.html\" title=\"struct objc2::runtime::AnyProtocol\"><code>AnyProtocol</code></a>.</p>\n</div></details></div></details>",0,"objc2::declare::ProtocolDecl"],["<section id=\"impl-Send-for-ProtocolBuilder\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#469\">Source</a><a href=\"#impl-Send-for-ProtocolBuilder\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\">ProtocolBuilder</a></h3></section>","Send","objc2::declare::ProtocolDecl"],["<section id=\"impl-Sync-for-ProtocolBuilder\" class=\"impl\"><a class=\"src rightside\" href=\"src/objc2/runtime/define.rs.html#470\">Source</a><a href=\"#impl-Sync-for-ProtocolBuilder\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"objc2/runtime/struct.ProtocolBuilder.html\" title=\"struct objc2::runtime::ProtocolBuilder\">ProtocolBuilder</a></h3></section>","Sync","objc2::declare::ProtocolDecl"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[9412]}