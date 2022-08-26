pub struct CrcHelper {}

impl CrcHelper {
	/*
		e.g. ruby soundbank
				soundName = items[0].chomp.strip.upcase.gsub( /\W/, '_' )
				soundId = Zlib.crc32( soundName )

	*/
	pub fn clean_name_from_name_upcase_underscore(name: &str) -> String {
		let upcase_name = name.to_uppercase();
		// Ruby: .gsub( /\W\./, ' ' ) // should be 'a-zA-Z0-9_', but actual code behaves differently
		let clean_name: String = upcase_name
			.chars()
			.map(|c| match c {
				'-' => '_', // match the ruby version
				'0'..='9' => c,
				'A'..='Z' => c,
				//			'a'..='z' => c,	// already uppercase
				'!'..='@' => c,
				'['..='`' => c,
				'{'..='~' => c,
				//		0x7f => c,			// ignore DEL
				_ => '_',
			})
			.collect();

		clean_name
	}
	pub fn crc_from_name_upcase_underscore(name: &str) -> u32 {
		let clean_name = CrcHelper::clean_name_from_name_upcase_underscore(name);
		println!("clean_name: {:?}", clean_name);
		const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
		let crc = CRC32.checksum(clean_name.as_bytes());

		crc
	}
	/*
		e.g. ruby packer
				name = filename.to_s
				name = name.downcase.gsub( /\W\./, ' ' )
	#			puts ">>"+name+"<<"

				@crc = Zlib::crc32( name.downcase )
	*/
	pub fn crc_from_name(name: &str) -> u32 {
		let downcase_name = name.to_lowercase();
		// Ruby: .gsub( /\W\./, ' ' ) // should be 'a-zA-Z0-9_', but actual code behaves differently
		let clean_name: String = downcase_name
			.chars()
			.map(|c| match c {
				'0'..='9' => c,
				'a'..='z' => c,
				//			'A'..='Z' => c,	// already downcase
				'!'..='@' => c,
				'['..='`' => c,
				'{'..='~' => c,
				//		0x7f => c,			// ignore DEL
				_ => ' ',
			})
			.collect();
		const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
		let crc = CRC32.checksum(clean_name.as_bytes());

		crc
	}
}
