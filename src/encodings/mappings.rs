use super::glyphnames::Glyph;

pub const MAC_ROMAN_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclam),
	Some(Glyph::quotedbl),
	Some(Glyph::numbersign),
	Some(Glyph::dollar),
	Some(Glyph::percent),
	Some(Glyph::ampersand),
	Some(Glyph::quotesingle),
	Some(Glyph::parenleft),
	Some(Glyph::parenright),
	Some(Glyph::asterisk),
	Some(Glyph::plus),
	Some(Glyph::comma),
	Some(Glyph::hyphen),
	Some(Glyph::period),
	Some(Glyph::slash),
	Some(Glyph::zero),
	Some(Glyph::one),
	Some(Glyph::two),
	Some(Glyph::three),
	Some(Glyph::four),
	Some(Glyph::five),
	Some(Glyph::six),
	Some(Glyph::seven),
	Some(Glyph::eight),
	Some(Glyph::nine),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	Some(Glyph::less),
	Some(Glyph::equal),
	Some(Glyph::greater),
	Some(Glyph::question),
	Some(Glyph::at),
	Some(Glyph::A),
	Some(Glyph::B),
	Some(Glyph::C),
	Some(Glyph::D),
	Some(Glyph::E),
	Some(Glyph::F),
	Some(Glyph::G),
	Some(Glyph::H),
	Some(Glyph::I),
	Some(Glyph::J),
	Some(Glyph::K),
	Some(Glyph::L),
	Some(Glyph::M),
	Some(Glyph::N),
	Some(Glyph::O),
	Some(Glyph::P),
	Some(Glyph::Q),
	Some(Glyph::R),
	Some(Glyph::S),
	Some(Glyph::T),
	Some(Glyph::U),
	Some(Glyph::V),
	Some(Glyph::W),
	Some(Glyph::X),
	Some(Glyph::Y),
	Some(Glyph::Z),
	Some(Glyph::bracketleft),
	Some(Glyph::backslash),
	Some(Glyph::bracketright),
	Some(Glyph::asciicircum),
	Some(Glyph::underscore),
	Some(Glyph::grave),
	Some(Glyph::a),
	Some(Glyph::b),
	Some(Glyph::c),
	Some(Glyph::d),
	Some(Glyph::e),
	Some(Glyph::f),
	Some(Glyph::g),
	Some(Glyph::h),
	Some(Glyph::i),
	Some(Glyph::j),
	Some(Glyph::k),
	Some(Glyph::l),
	Some(Glyph::m),
	Some(Glyph::n),
	Some(Glyph::o),
	Some(Glyph::p),
	Some(Glyph::q),
	Some(Glyph::r),
	Some(Glyph::s),
	Some(Glyph::t),
	Some(Glyph::u),
	Some(Glyph::v),
	Some(Glyph::w),
	Some(Glyph::x),
	Some(Glyph::y),
	Some(Glyph::z),
	Some(Glyph::braceleft),
	Some(Glyph::bar),
	Some(Glyph::braceright),
	Some(Glyph::asciitilde),
	None,
	Some(Glyph::Adieresis),
	Some(Glyph::Aring),
	Some(Glyph::Ccedilla),
	Some(Glyph::Eacute),
	Some(Glyph::Ntilde),
	Some(Glyph::Odieresis),
	Some(Glyph::Udieresis),
	Some(Glyph::aacute),
	Some(Glyph::agrave),
	Some(Glyph::acircumflex),
	Some(Glyph::adieresis),
	Some(Glyph::atilde),
	Some(Glyph::aring),
	Some(Glyph::ccedilla),
	Some(Glyph::eacute),
	Some(Glyph::egrave),
	Some(Glyph::ecircumflex),
	Some(Glyph::edieresis),
	Some(Glyph::iacute),
	Some(Glyph::igrave),
	Some(Glyph::icircumflex),
	Some(Glyph::idieresis),
	Some(Glyph::ntilde),
	Some(Glyph::oacute),
	Some(Glyph::ograve),
	Some(Glyph::ocircumflex),
	Some(Glyph::odieresis),
	Some(Glyph::otilde),
	Some(Glyph::uacute),
	Some(Glyph::ugrave),
	Some(Glyph::ucircumflex),
	Some(Glyph::udieresis),
	Some(Glyph::dagger),
	Some(Glyph::degree),
	Some(Glyph::cent),
	Some(Glyph::sterling),
	Some(Glyph::section),
	Some(Glyph::bullet),
	Some(Glyph::paragraph),
	Some(Glyph::germandbls),
	Some(Glyph::registered),
	Some(Glyph::copyright),
	Some(Glyph::trademark),
	Some(Glyph::acute),
	Some(Glyph::dieresis),
	Some(Glyph::notequal),
	Some(Glyph::AE),
	Some(Glyph::Oslash),
	Some(Glyph::infinity),
	Some(Glyph::plusminus),
	Some(Glyph::lessequal),
	Some(Glyph::greaterequal),
	Some(Glyph::yen),
	Some(Glyph::mu),
	Some(Glyph::partialdiff),
	Some(Glyph::summation),
	Some(Glyph::product),
	Some(Glyph::pi),
	Some(Glyph::integral),
	Some(Glyph::ordfeminine),
	Some(Glyph::ordmasculine),
	Some(Glyph::Omega),
	Some(Glyph::ae),
	Some(Glyph::oslash),
	Some(Glyph::questiondown),
	Some(Glyph::exclamdown),
	Some(Glyph::logicalnot),
	Some(Glyph::radical),
	Some(Glyph::florin),
	Some(Glyph::approxequal),
	Some(Glyph::Delta),
	Some(Glyph::guillemotleft),
	Some(Glyph::guillemotright),
	Some(Glyph::ellipsis),
	Some(Glyph::space),
	Some(Glyph::Agrave),
	Some(Glyph::Atilde),
	Some(Glyph::Otilde),
	Some(Glyph::OE),
	Some(Glyph::oe),
	Some(Glyph::endash),
	Some(Glyph::emdash),
	Some(Glyph::quotedblleft),
	Some(Glyph::quotedblright),
	Some(Glyph::quoteleft),
	Some(Glyph::quoteright),
	Some(Glyph::divide),
	Some(Glyph::lozenge),
	Some(Glyph::ydieresis),
	Some(Glyph::Ydieresis),
	Some(Glyph::fraction),
	Some(Glyph::currency),
	Some(Glyph::guilsinglleft),
	Some(Glyph::guilsinglright),
	Some(Glyph::fi),
	Some(Glyph::fl),
	Some(Glyph::daggerdbl),
	Some(Glyph::periodcentered),
	Some(Glyph::quotesinglbase),
	Some(Glyph::quotedblbase),
	Some(Glyph::perthousand),
	Some(Glyph::Acircumflex),
	Some(Glyph::Ecircumflex),
	Some(Glyph::Aacute),
	Some(Glyph::Edieresis),
	Some(Glyph::Egrave),
	Some(Glyph::Iacute),
	Some(Glyph::Icircumflex),
	Some(Glyph::Idieresis),
	Some(Glyph::Igrave),
	Some(Glyph::Oacute),
	Some(Glyph::Ocircumflex),
	Some(Glyph::apple),
	Some(Glyph::Ograve),
	Some(Glyph::Uacute),
	Some(Glyph::Ucircumflex),
	Some(Glyph::Ugrave),
	Some(Glyph::dotlessi),
	Some(Glyph::circumflex),
	Some(Glyph::tilde),
	Some(Glyph::macron),
	Some(Glyph::breve),
	Some(Glyph::dotaccent),
	Some(Glyph::ring),
	Some(Glyph::cedilla),
	Some(Glyph::hungarumlaut),
	Some(Glyph::ogonek),
	Some(Glyph::caron),
];

pub const MAC_EXPERT_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclamsmall),
	Some(Glyph::Hungarumlautsmall),
	Some(Glyph::centoldstyle),
	Some(Glyph::dollaroldstyle),
	Some(Glyph::dollarsuperior),
	Some(Glyph::ampersandsmall),
	Some(Glyph::Acutesmall),
	Some(Glyph::parenleftsuperior),
	Some(Glyph::parenrightsuperior),
	Some(Glyph::twodotenleader),
	Some(Glyph::onedotenleader),
	Some(Glyph::comma),
	Some(Glyph::hyphen),
	Some(Glyph::period),
	Some(Glyph::fraction),
	Some(Glyph::zerooldstyle),
	Some(Glyph::oneoldstyle),
	Some(Glyph::twooldstyle),
	Some(Glyph::threeoldstyle),
	Some(Glyph::fouroldstyle),
	Some(Glyph::fiveoldstyle),
	Some(Glyph::sixoldstyle),
	Some(Glyph::sevenoldstyle),
	Some(Glyph::eightoldstyle),
	Some(Glyph::nineoldstyle),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	None,
	Some(Glyph::threequartersemdash),
	None,
	Some(Glyph::questionsmall),
	None,
	None,
	None,
	None,
	Some(Glyph::Ethsmall),
	None,
	None,
	Some(Glyph::onequarter),
	Some(Glyph::onehalf),
	Some(Glyph::threequarters),
	Some(Glyph::oneeighth),
	Some(Glyph::threeeighths),
	Some(Glyph::fiveeighths),
	Some(Glyph::seveneighths),
	Some(Glyph::onethird),
	Some(Glyph::twothirds),
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::ff),
	Some(Glyph::fi),
	Some(Glyph::fl),
	Some(Glyph::ffi),
	Some(Glyph::ffl),
	Some(Glyph::parenleftinferior),
	None,
	Some(Glyph::parenrightinferior),
	Some(Glyph::Circumflexsmall),
	Some(Glyph::hypheninferior),
	Some(Glyph::Gravesmall),
	Some(Glyph::Asmall),
	Some(Glyph::Bsmall),
	Some(Glyph::Csmall),
	Some(Glyph::Dsmall),
	Some(Glyph::Esmall),
	Some(Glyph::Fsmall),
	Some(Glyph::Gsmall),
	Some(Glyph::Hsmall),
	Some(Glyph::Ismall),
	Some(Glyph::Jsmall),
	Some(Glyph::Ksmall),
	Some(Glyph::Lsmall),
	Some(Glyph::Msmall),
	Some(Glyph::Nsmall),
	Some(Glyph::Osmall),
	Some(Glyph::Psmall),
	Some(Glyph::Qsmall),
	Some(Glyph::Rsmall),
	Some(Glyph::Ssmall),
	Some(Glyph::Tsmall),
	Some(Glyph::Usmall),
	Some(Glyph::Vsmall),
	Some(Glyph::Wsmall),
	Some(Glyph::Xsmall),
	Some(Glyph::Ysmall),
	Some(Glyph::Zsmall),
	Some(Glyph::colonmonetary),
	Some(Glyph::onefitted),
	Some(Glyph::rupiah),
	Some(Glyph::Tildesmall),
	None,
	None,
	Some(Glyph::asuperior),
	Some(Glyph::centsuperior),
	None,
	None,
	None,
	None,
	Some(Glyph::Aacutesmall),
	Some(Glyph::Agravesmall),
	Some(Glyph::Acircumflexsmall),
	Some(Glyph::Adieresissmall),
	Some(Glyph::Atildesmall),
	Some(Glyph::Aringsmall),
	Some(Glyph::Ccedillasmall),
	Some(Glyph::Eacutesmall),
	Some(Glyph::Egravesmall),
	Some(Glyph::Ecircumflexsmall),
	Some(Glyph::Edieresissmall),
	Some(Glyph::Iacutesmall),
	Some(Glyph::Igravesmall),
	Some(Glyph::Icircumflexsmall),
	Some(Glyph::Idieresissmall),
	Some(Glyph::Ntildesmall),
	Some(Glyph::Oacutesmall),
	Some(Glyph::Ogravesmall),
	Some(Glyph::Ocircumflexsmall),
	Some(Glyph::Odieresissmall),
	Some(Glyph::Otildesmall),
	Some(Glyph::Uacutesmall),
	Some(Glyph::Ugravesmall),
	Some(Glyph::Ucircumflexsmall),
	Some(Glyph::Udieresissmall),
	None,
	Some(Glyph::eightsuperior),
	Some(Glyph::fourinferior),
	Some(Glyph::threeinferior),
	Some(Glyph::sixinferior),
	Some(Glyph::eightinferior),
	Some(Glyph::seveninferior),
	Some(Glyph::Scaronsmall),
	None,
	Some(Glyph::centinferior),
	Some(Glyph::twoinferior),
	None,
	Some(Glyph::Dieresissmall),
	None,
	Some(Glyph::Caronsmall),
	Some(Glyph::osuperior),
	Some(Glyph::fiveinferior),
	None,
	Some(Glyph::commainferior),
	Some(Glyph::periodinferior),
	Some(Glyph::Yacutesmall),
	None,
	Some(Glyph::dollarinferior),
	None,
	None,
	Some(Glyph::Thornsmall),
	None,
	Some(Glyph::nineinferior),
	Some(Glyph::zeroinferior),
	Some(Glyph::Zcaronsmall),
	Some(Glyph::AEsmall),
	Some(Glyph::Oslashsmall),
	Some(Glyph::questiondownsmall),
	Some(Glyph::oneinferior),
	Some(Glyph::Lslashsmall),
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::Cedillasmall),
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::OEsmall),
	Some(Glyph::figuredash),
	Some(Glyph::hyphensuperior),
	None,
	None,
	None,
	None,
	Some(Glyph::exclamdownsmall),
	None,
	Some(Glyph::Ydieresissmall),
	None,
	Some(Glyph::onesuperior),
	Some(Glyph::twosuperior),
	Some(Glyph::threesuperior),
	Some(Glyph::foursuperior),
	Some(Glyph::fivesuperior),
	Some(Glyph::sixsuperior),
	Some(Glyph::sevensuperior),
	Some(Glyph::ninesuperior),
	Some(Glyph::zerosuperior),
	None,
	Some(Glyph::esuperior),
	Some(Glyph::rsuperior),
	Some(Glyph::tsuperior),
	None,
	None,
	Some(Glyph::isuperior),
	Some(Glyph::ssuperior),
	Some(Glyph::dsuperior),
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::lsuperior),
	Some(Glyph::Ogoneksmall),
	Some(Glyph::Brevesmall),
	Some(Glyph::Macronsmall),
	Some(Glyph::bsuperior),
	Some(Glyph::nsuperior),
	Some(Glyph::msuperior),
	Some(Glyph::commasuperior),
	Some(Glyph::periodsuperior),
	Some(Glyph::Dotaccentsmall),
	Some(Glyph::Ringsmall),
	None,
	None,
	None,
	None,
];

pub const WIN_ANSI_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclam),
	Some(Glyph::quotedbl),
	Some(Glyph::numbersign),
	Some(Glyph::dollar),
	Some(Glyph::percent),
	Some(Glyph::ampersand),
	Some(Glyph::quotesingle),
	Some(Glyph::parenleft),
	Some(Glyph::parenright),
	Some(Glyph::asterisk),
	Some(Glyph::plus),
	Some(Glyph::comma),
	Some(Glyph::hyphen),
	Some(Glyph::period),
	Some(Glyph::slash),
	Some(Glyph::zero),
	Some(Glyph::one),
	Some(Glyph::two),
	Some(Glyph::three),
	Some(Glyph::four),
	Some(Glyph::five),
	Some(Glyph::six),
	Some(Glyph::seven),
	Some(Glyph::eight),
	Some(Glyph::nine),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	Some(Glyph::less),
	Some(Glyph::equal),
	Some(Glyph::greater),
	Some(Glyph::question),
	Some(Glyph::at),
	Some(Glyph::A),
	Some(Glyph::B),
	Some(Glyph::C),
	Some(Glyph::D),
	Some(Glyph::E),
	Some(Glyph::F),
	Some(Glyph::G),
	Some(Glyph::H),
	Some(Glyph::I),
	Some(Glyph::J),
	Some(Glyph::K),
	Some(Glyph::L),
	Some(Glyph::M),
	Some(Glyph::N),
	Some(Glyph::O),
	Some(Glyph::P),
	Some(Glyph::Q),
	Some(Glyph::R),
	Some(Glyph::S),
	Some(Glyph::T),
	Some(Glyph::U),
	Some(Glyph::V),
	Some(Glyph::W),
	Some(Glyph::X),
	Some(Glyph::Y),
	Some(Glyph::Z),
	Some(Glyph::bracketleft),
	Some(Glyph::backslash),
	Some(Glyph::bracketright),
	Some(Glyph::asciicircum),
	Some(Glyph::underscore),
	Some(Glyph::grave),
	Some(Glyph::a),
	Some(Glyph::b),
	Some(Glyph::c),
	Some(Glyph::d),
	Some(Glyph::e),
	Some(Glyph::f),
	Some(Glyph::g),
	Some(Glyph::h),
	Some(Glyph::i),
	Some(Glyph::j),
	Some(Glyph::k),
	Some(Glyph::l),
	Some(Glyph::m),
	Some(Glyph::n),
	Some(Glyph::o),
	Some(Glyph::p),
	Some(Glyph::q),
	Some(Glyph::r),
	Some(Glyph::s),
	Some(Glyph::t),
	Some(Glyph::u),
	Some(Glyph::v),
	Some(Glyph::w),
	Some(Glyph::x),
	Some(Glyph::y),
	Some(Glyph::z),
	Some(Glyph::braceleft),
	Some(Glyph::bar),
	Some(Glyph::braceright),
	Some(Glyph::asciitilde),
	Some(Glyph::bullet),
	Some(Glyph::Euro),
	Some(Glyph::bullet),
	Some(Glyph::quotesinglbase),
	Some(Glyph::florin),
	Some(Glyph::quotedblbase),
	Some(Glyph::ellipsis),
	Some(Glyph::dagger),
	Some(Glyph::daggerdbl),
	Some(Glyph::circumflex),
	Some(Glyph::perthousand),
	Some(Glyph::Scaron),
	Some(Glyph::guilsinglleft),
	Some(Glyph::OE),
	Some(Glyph::bullet),
	Some(Glyph::Zcaron),
	Some(Glyph::bullet),
	Some(Glyph::bullet),
	Some(Glyph::quoteleft),
	Some(Glyph::quoteright),
	Some(Glyph::quotedblleft),
	Some(Glyph::quotedblright),
	Some(Glyph::bullet),
	Some(Glyph::endash),
	Some(Glyph::emdash),
	Some(Glyph::tilde),
	Some(Glyph::trademark),
	Some(Glyph::scaron),
	Some(Glyph::guilsinglright),
	Some(Glyph::oe),
	Some(Glyph::bullet),
	Some(Glyph::zcaron),
	Some(Glyph::Ydieresis),
	Some(Glyph::space),
	Some(Glyph::exclamdown),
	Some(Glyph::cent),
	Some(Glyph::sterling),
	Some(Glyph::currency),
	Some(Glyph::yen),
	Some(Glyph::brokenbar),
	Some(Glyph::section),
	Some(Glyph::dieresis),
	Some(Glyph::copyright),
	Some(Glyph::ordfeminine),
	Some(Glyph::guillemotleft),
	Some(Glyph::logicalnot),
	Some(Glyph::hyphen),
	Some(Glyph::registered),
	Some(Glyph::macron),
	Some(Glyph::degree),
	Some(Glyph::plusminus),
	Some(Glyph::twosuperior),
	Some(Glyph::threesuperior),
	Some(Glyph::acute),
	Some(Glyph::mu),
	Some(Glyph::paragraph),
	Some(Glyph::periodcentered),
	Some(Glyph::cedilla),
	Some(Glyph::onesuperior),
	Some(Glyph::ordmasculine),
	Some(Glyph::guillemotright),
	Some(Glyph::onequarter),
	Some(Glyph::onehalf),
	Some(Glyph::threequarters),
	Some(Glyph::questiondown),
	Some(Glyph::Agrave),
	Some(Glyph::Aacute),
	Some(Glyph::Acircumflex),
	Some(Glyph::Atilde),
	Some(Glyph::Adieresis),
	Some(Glyph::Aring),
	Some(Glyph::AE),
	Some(Glyph::Ccedilla),
	Some(Glyph::Egrave),
	Some(Glyph::Eacute),
	Some(Glyph::Ecircumflex),
	Some(Glyph::Edieresis),
	Some(Glyph::Igrave),
	Some(Glyph::Iacute),
	Some(Glyph::Icircumflex),
	Some(Glyph::Idieresis),
	Some(Glyph::Eth),
	Some(Glyph::Ntilde),
	Some(Glyph::Ograve),
	Some(Glyph::Oacute),
	Some(Glyph::Ocircumflex),
	Some(Glyph::Otilde),
	Some(Glyph::Odieresis),
	Some(Glyph::multiply),
	Some(Glyph::Oslash),
	Some(Glyph::Ugrave),
	Some(Glyph::Uacute),
	Some(Glyph::Ucircumflex),
	Some(Glyph::Udieresis),
	Some(Glyph::Yacute),
	Some(Glyph::Thorn),
	Some(Glyph::germandbls),
	Some(Glyph::agrave),
	Some(Glyph::aacute),
	Some(Glyph::acircumflex),
	Some(Glyph::atilde),
	Some(Glyph::adieresis),
	Some(Glyph::aring),
	Some(Glyph::ae),
	Some(Glyph::ccedilla),
	Some(Glyph::egrave),
	Some(Glyph::eacute),
	Some(Glyph::ecircumflex),
	Some(Glyph::edieresis),
	Some(Glyph::igrave),
	Some(Glyph::iacute),
	Some(Glyph::icircumflex),
	Some(Glyph::idieresis),
	Some(Glyph::eth),
	Some(Glyph::ntilde),
	Some(Glyph::ograve),
	Some(Glyph::oacute),
	Some(Glyph::ocircumflex),
	Some(Glyph::otilde),
	Some(Glyph::odieresis),
	Some(Glyph::divide),
	Some(Glyph::oslash),
	Some(Glyph::ugrave),
	Some(Glyph::uacute),
	Some(Glyph::ucircumflex),
	Some(Glyph::udieresis),
	Some(Glyph::yacute),
	Some(Glyph::thorn),
	Some(Glyph::ydieresis),
];

pub const STANDARD_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclam),
	Some(Glyph::quotedbl),
	Some(Glyph::numbersign),
	Some(Glyph::dollar),
	Some(Glyph::percent),
	Some(Glyph::ampersand),
	Some(Glyph::quoteright),
	Some(Glyph::parenleft),
	Some(Glyph::parenright),
	Some(Glyph::asterisk),
	Some(Glyph::plus),
	Some(Glyph::comma),
	Some(Glyph::hyphen),
	Some(Glyph::period),
	Some(Glyph::slash),
	Some(Glyph::zero),
	Some(Glyph::one),
	Some(Glyph::two),
	Some(Glyph::three),
	Some(Glyph::four),
	Some(Glyph::five),
	Some(Glyph::six),
	Some(Glyph::seven),
	Some(Glyph::eight),
	Some(Glyph::nine),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	Some(Glyph::less),
	Some(Glyph::equal),
	Some(Glyph::greater),
	Some(Glyph::question),
	Some(Glyph::at),
	Some(Glyph::A),
	Some(Glyph::B),
	Some(Glyph::C),
	Some(Glyph::D),
	Some(Glyph::E),
	Some(Glyph::F),
	Some(Glyph::G),
	Some(Glyph::H),
	Some(Glyph::I),
	Some(Glyph::J),
	Some(Glyph::K),
	Some(Glyph::L),
	Some(Glyph::M),
	Some(Glyph::N),
	Some(Glyph::O),
	Some(Glyph::P),
	Some(Glyph::Q),
	Some(Glyph::R),
	Some(Glyph::S),
	Some(Glyph::T),
	Some(Glyph::U),
	Some(Glyph::V),
	Some(Glyph::W),
	Some(Glyph::X),
	Some(Glyph::Y),
	Some(Glyph::Z),
	Some(Glyph::bracketleft),
	Some(Glyph::backslash),
	Some(Glyph::bracketright),
	Some(Glyph::asciicircum),
	Some(Glyph::underscore),
	Some(Glyph::quoteleft),
	Some(Glyph::a),
	Some(Glyph::b),
	Some(Glyph::c),
	Some(Glyph::d),
	Some(Glyph::e),
	Some(Glyph::f),
	Some(Glyph::g),
	Some(Glyph::h),
	Some(Glyph::i),
	Some(Glyph::j),
	Some(Glyph::k),
	Some(Glyph::l),
	Some(Glyph::m),
	Some(Glyph::n),
	Some(Glyph::o),
	Some(Glyph::p),
	Some(Glyph::q),
	Some(Glyph::r),
	Some(Glyph::s),
	Some(Glyph::t),
	Some(Glyph::u),
	Some(Glyph::v),
	Some(Glyph::w),
	Some(Glyph::x),
	Some(Glyph::y),
	Some(Glyph::z),
	Some(Glyph::braceleft),
	Some(Glyph::bar),
	Some(Glyph::braceright),
	Some(Glyph::asciitilde),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::exclamdown),
	Some(Glyph::cent),
	Some(Glyph::sterling),
	Some(Glyph::fraction),
	Some(Glyph::yen),
	Some(Glyph::florin),
	Some(Glyph::section),
	Some(Glyph::currency),
	Some(Glyph::quotesingle),
	Some(Glyph::quotedblleft),
	Some(Glyph::guillemotleft),
	Some(Glyph::guilsinglleft),
	Some(Glyph::guilsinglright),
	Some(Glyph::fi),
	Some(Glyph::fl),
	None,
	Some(Glyph::endash),
	Some(Glyph::dagger),
	Some(Glyph::daggerdbl),
	Some(Glyph::periodcentered),
	None,
	Some(Glyph::paragraph),
	Some(Glyph::bullet),
	Some(Glyph::quotesinglbase),
	Some(Glyph::quotedblbase),
	Some(Glyph::quotedblright),
	Some(Glyph::guillemotright),
	Some(Glyph::ellipsis),
	Some(Glyph::perthousand),
	None,
	Some(Glyph::questiondown),
	None,
	Some(Glyph::grave),
	Some(Glyph::acute),
	Some(Glyph::circumflex),
	Some(Glyph::tilde),
	Some(Glyph::macron),
	Some(Glyph::breve),
	Some(Glyph::dotaccent),
	Some(Glyph::dieresis),
	None,
	Some(Glyph::ring),
	Some(Glyph::cedilla),
	None,
	Some(Glyph::hungarumlaut),
	Some(Glyph::ogonek),
	Some(Glyph::caron),
	Some(Glyph::emdash),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::AE),
	None,
	Some(Glyph::ordfeminine),
	None,
	None,
	None,
	None,
	Some(Glyph::Lslash),
	Some(Glyph::Oslash),
	Some(Glyph::OE),
	Some(Glyph::ordmasculine),
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::ae),
	None,
	None,
	None,
	Some(Glyph::dotlessi),
	None,
	None,
	Some(Glyph::lslash),
	Some(Glyph::oslash),
	Some(Glyph::oe),
	Some(Glyph::germandbls),
	None,
	None,
	None,
	None,
];

#[allow(dead_code)]
pub const EXPERT_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclamsmall),
	Some(Glyph::Hungarumlautsmall),
	None,
	Some(Glyph::dollaroldstyle),
	Some(Glyph::dollarsuperior),
	Some(Glyph::ampersandsmall),
	Some(Glyph::Acutesmall),
	Some(Glyph::parenleftsuperior),
	Some(Glyph::parenrightsuperior),
	Some(Glyph::twodotenleader),
	Some(Glyph::onedotenleader),
	Some(Glyph::comma),
	Some(Glyph::hyphen),
	Some(Glyph::period),
	Some(Glyph::fraction),
	Some(Glyph::zerooldstyle),
	Some(Glyph::oneoldstyle),
	Some(Glyph::twooldstyle),
	Some(Glyph::threeoldstyle),
	Some(Glyph::fouroldstyle),
	Some(Glyph::fiveoldstyle),
	Some(Glyph::sixoldstyle),
	Some(Glyph::sevenoldstyle),
	Some(Glyph::eightoldstyle),
	Some(Glyph::nineoldstyle),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	Some(Glyph::commasuperior),
	Some(Glyph::threequartersemdash),
	Some(Glyph::periodsuperior),
	Some(Glyph::questionsmall),
	None,
	Some(Glyph::asuperior),
	Some(Glyph::bsuperior),
	Some(Glyph::centsuperior),
	Some(Glyph::dsuperior),
	Some(Glyph::esuperior),
	None,
	None,
	None,
	Some(Glyph::isuperior),
	None,
	None,
	Some(Glyph::lsuperior),
	Some(Glyph::msuperior),
	Some(Glyph::nsuperior),
	Some(Glyph::osuperior),
	None,
	None,
	Some(Glyph::rsuperior),
	Some(Glyph::ssuperior),
	Some(Glyph::tsuperior),
	None,
	Some(Glyph::ff),
	Some(Glyph::fi),
	Some(Glyph::fl),
	Some(Glyph::ffi),
	Some(Glyph::ffl),
	Some(Glyph::parenleftinferior),
	None,
	Some(Glyph::parenrightinferior),
	Some(Glyph::Circumflexsmall),
	Some(Glyph::hyphensuperior),
	Some(Glyph::Gravesmall),
	Some(Glyph::Asmall),
	Some(Glyph::Bsmall),
	Some(Glyph::Csmall),
	Some(Glyph::Dsmall),
	Some(Glyph::Esmall),
	Some(Glyph::Fsmall),
	Some(Glyph::Gsmall),
	Some(Glyph::Hsmall),
	Some(Glyph::Ismall),
	Some(Glyph::Jsmall),
	Some(Glyph::Ksmall),
	Some(Glyph::Lsmall),
	Some(Glyph::Msmall),
	Some(Glyph::Nsmall),
	Some(Glyph::Osmall),
	Some(Glyph::Psmall),
	Some(Glyph::Qsmall),
	Some(Glyph::Rsmall),
	Some(Glyph::Ssmall),
	Some(Glyph::Tsmall),
	Some(Glyph::Usmall),
	Some(Glyph::Vsmall),
	Some(Glyph::Wsmall),
	Some(Glyph::Xsmall),
	Some(Glyph::Ysmall),
	Some(Glyph::Zsmall),
	Some(Glyph::colonmonetary),
	Some(Glyph::onefitted),
	Some(Glyph::rupiah),
	Some(Glyph::Tildesmall),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::exclamdownsmall),
	Some(Glyph::centoldstyle),
	Some(Glyph::Lslashsmall),
	None,
	None,
	Some(Glyph::Scaronsmall),
	Some(Glyph::Zcaronsmall),
	Some(Glyph::Dieresissmall),
	Some(Glyph::Brevesmall),
	Some(Glyph::Caronsmall),
	None,
	Some(Glyph::Dotaccentsmall),
	None,
	None,
	Some(Glyph::Macronsmall),
	None,
	None,
	Some(Glyph::figuredash),
	Some(Glyph::hypheninferior),
	None,
	None,
	Some(Glyph::Ogoneksmall),
	Some(Glyph::Ringsmall),
	Some(Glyph::Cedillasmall),
	None,
	None,
	None,
	Some(Glyph::onequarter),
	Some(Glyph::onehalf),
	Some(Glyph::threequarters),
	Some(Glyph::questiondownsmall),
	Some(Glyph::oneeighth),
	Some(Glyph::threeeighths),
	Some(Glyph::fiveeighths),
	Some(Glyph::seveneighths),
	Some(Glyph::onethird),
	Some(Glyph::twothirds),
	None,
	None,
	Some(Glyph::zerosuperior),
	Some(Glyph::onesuperior),
	Some(Glyph::twosuperior),
	Some(Glyph::threesuperior),
	Some(Glyph::foursuperior),
	Some(Glyph::fivesuperior),
	Some(Glyph::sixsuperior),
	Some(Glyph::sevensuperior),
	Some(Glyph::eightsuperior),
	Some(Glyph::ninesuperior),
	Some(Glyph::zeroinferior),
	Some(Glyph::oneinferior),
	Some(Glyph::twoinferior),
	Some(Glyph::threeinferior),
	Some(Glyph::fourinferior),
	Some(Glyph::fiveinferior),
	Some(Glyph::sixinferior),
	Some(Glyph::seveninferior),
	Some(Glyph::eightinferior),
	Some(Glyph::nineinferior),
	Some(Glyph::centinferior),
	Some(Glyph::dollarinferior),
	Some(Glyph::periodinferior),
	Some(Glyph::commainferior),
	Some(Glyph::Agravesmall),
	Some(Glyph::Aacutesmall),
	Some(Glyph::Acircumflexsmall),
	Some(Glyph::Atildesmall),
	Some(Glyph::Adieresissmall),
	Some(Glyph::Aringsmall),
	Some(Glyph::AEsmall),
	Some(Glyph::Ccedillasmall),
	Some(Glyph::Egravesmall),
	Some(Glyph::Eacutesmall),
	Some(Glyph::Ecircumflexsmall),
	Some(Glyph::Edieresissmall),
	Some(Glyph::Igravesmall),
	Some(Glyph::Iacutesmall),
	Some(Glyph::Icircumflexsmall),
	Some(Glyph::Idieresissmall),
	Some(Glyph::Ethsmall),
	Some(Glyph::Ntildesmall),
	Some(Glyph::Ogravesmall),
	Some(Glyph::Oacutesmall),
	Some(Glyph::Ocircumflexsmall),
	Some(Glyph::Otildesmall),
	Some(Glyph::Odieresissmall),
	Some(Glyph::OEsmall),
	Some(Glyph::Oslashsmall),
	Some(Glyph::Ugravesmall),
	Some(Glyph::Uacutesmall),
	Some(Glyph::Ucircumflexsmall),
	Some(Glyph::Udieresissmall),
	Some(Glyph::Yacutesmall),
	Some(Glyph::Thornsmall),
	Some(Glyph::Ydieresissmall),
];

#[allow(dead_code)]
pub const SYMBOL_ENCODING: [Option<u16>; 256] = [
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::space),
	Some(Glyph::exclam),
	Some(Glyph::universal),
	Some(Glyph::numbersign),
	Some(Glyph::existential),
	Some(Glyph::percent),
	Some(Glyph::ampersand),
	Some(Glyph::suchthat),
	Some(Glyph::parenleft),
	Some(Glyph::parenright),
	Some(Glyph::asteriskmath),
	Some(Glyph::plus),
	Some(Glyph::comma),
	Some(Glyph::minus),
	Some(Glyph::period),
	Some(Glyph::slash),
	Some(Glyph::zero),
	Some(Glyph::one),
	Some(Glyph::two),
	Some(Glyph::three),
	Some(Glyph::four),
	Some(Glyph::five),
	Some(Glyph::six),
	Some(Glyph::seven),
	Some(Glyph::eight),
	Some(Glyph::nine),
	Some(Glyph::colon),
	Some(Glyph::semicolon),
	Some(Glyph::less),
	Some(Glyph::equal),
	Some(Glyph::greater),
	Some(Glyph::question),
	Some(Glyph::congruent),
	Some(Glyph::Alpha),
	Some(Glyph::Beta),
	Some(Glyph::Chi),
	Some(Glyph::Delta),
	Some(Glyph::Epsilon),
	Some(Glyph::Phi),
	Some(Glyph::Gamma),
	Some(Glyph::Eta),
	Some(Glyph::Iota),
	Some(Glyph::theta1),
	Some(Glyph::Kappa),
	Some(Glyph::Lambda),
	Some(Glyph::Mu),
	Some(Glyph::Nu),
	Some(Glyph::Omicron),
	Some(Glyph::Pi),
	Some(Glyph::Theta),
	Some(Glyph::Rho),
	Some(Glyph::Sigma),
	Some(Glyph::Tau),
	Some(Glyph::Upsilon),
	Some(Glyph::sigma1),
	Some(Glyph::Omega),
	Some(Glyph::Xi),
	Some(Glyph::Psi),
	Some(Glyph::Zeta),
	Some(Glyph::bracketleft),
	Some(Glyph::therefore),
	Some(Glyph::bracketright),
	Some(Glyph::perpendicular),
	Some(Glyph::underscore),
	Some(Glyph::radicalex),
	Some(Glyph::alpha),
	Some(Glyph::beta),
	Some(Glyph::chi),
	Some(Glyph::delta),
	Some(Glyph::epsilon),
	Some(Glyph::phi),
	Some(Glyph::gamma),
	Some(Glyph::eta),
	Some(Glyph::iota),
	Some(Glyph::phi2),
	Some(Glyph::kappa),
	Some(Glyph::lambda),
	Some(Glyph::mu),
	Some(Glyph::nu),
	Some(Glyph::omicron),
	Some(Glyph::pi),
	Some(Glyph::theta),
	Some(Glyph::rho),
	Some(Glyph::sigma),
	Some(Glyph::tau),
	Some(Glyph::upsilon),
	Some(Glyph::omega1),
	Some(Glyph::omega),
	Some(Glyph::xi),
	Some(Glyph::psi),
	Some(Glyph::zeta),
	Some(Glyph::braceleft),
	Some(Glyph::bar),
	Some(Glyph::braceright),
	Some(Glyph::similar),
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	None,
	Some(Glyph::Upsilon1),
	Some(Glyph::minute),
	Some(Glyph::lessequal),
	Some(Glyph::fraction),
	Some(Glyph::infinity),
	Some(Glyph::florin),
	Some(Glyph::club),
	Some(Glyph::diamond),
	Some(Glyph::heart),
	Some(Glyph::spade),
	Some(Glyph::arrowboth),
	Some(Glyph::arrowleft),
	Some(Glyph::arrowup),
	Some(Glyph::arrowright),
	Some(Glyph::arrowdown),
	Some(Glyph::degree),
	Some(Glyph::plusminus),
	Some(Glyph::second),
	Some(Glyph::greaterequal),
	Some(Glyph::multiply),
	Some(Glyph::proportional),
	Some(Glyph::partialdiff),
	Some(Glyph::bullet),
	Some(Glyph::divide),
	Some(Glyph::notequal),
	Some(Glyph::equivalence),
	Some(Glyph::approxequal),
	Some(Glyph::ellipsis),
	Some(Glyph::arrowvertex),
	Some(Glyph::arrowhorizex),
	Some(Glyph::carriagereturn),
	Some(Glyph::aleph),
	Some(Glyph::Ifraktur),
	Some(Glyph::Rfraktur),
	Some(Glyph::weierstrass),
	Some(Glyph::circlemultiply),
	Some(Glyph::circleplus),
	Some(Glyph::emptyset),
	Some(Glyph::intersection),
	Some(Glyph::union),
	Some(Glyph::propersuperset),
	Some(Glyph::reflexsuperset),
	Some(Glyph::notsubset),
	Some(Glyph::propersubset),
	Some(Glyph::reflexsubset),
	Some(Glyph::element),
	Some(Glyph::notelement),
	Some(Glyph::angle),
	Some(Glyph::gradient),
	Some(Glyph::registerserif),
	Some(Glyph::copyrightserif),
	Some(Glyph::trademarkserif),
	Some(Glyph::product),
	Some(Glyph::radical),
	Some(Glyph::dotmath),
	Some(Glyph::logicalnot),
	Some(Glyph::logicaland),
	Some(Glyph::logicalor),
	Some(Glyph::arrowdblboth),
	Some(Glyph::arrowdblleft),
	Some(Glyph::arrowdblup),
	Some(Glyph::arrowdblright),
	Some(Glyph::arrowdbldown),
	Some(Glyph::lozenge),
	Some(Glyph::angleleft),
	Some(Glyph::registersans),
	Some(Glyph::copyrightsans),
	Some(Glyph::trademarksans),
	Some(Glyph::summation),
	Some(Glyph::parenlefttp),
	Some(Glyph::parenleftex),
	Some(Glyph::parenleftbt),
	Some(Glyph::bracketlefttp),
	Some(Glyph::bracketleftex),
	Some(Glyph::bracketleftbt),
	Some(Glyph::bracelefttp),
	Some(Glyph::braceleftmid),
	Some(Glyph::braceleftbt),
	Some(Glyph::braceex),
	None,
	Some(Glyph::angleright),
	Some(Glyph::integral),
	Some(Glyph::integraltp),
	Some(Glyph::integralex),
	Some(Glyph::integralbt),
	Some(Glyph::parenrighttp),
	Some(Glyph::parenrightex),
	Some(Glyph::parenrightbt),
	Some(Glyph::bracketrighttp),
	Some(Glyph::bracketrightex),
	Some(Glyph::bracketrightbt),
	Some(Glyph::bracerighttp),
	Some(Glyph::bracerightmid),
	Some(Glyph::bracerightbt),
	None,
];