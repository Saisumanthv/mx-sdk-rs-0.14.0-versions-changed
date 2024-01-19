use super::BoxedBytes;
use crate::abi::TypeAbi;
use alloc::string::String;
use dharitri_codec::*;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// MOA is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenIdentifier(BoxedBytes);

impl TokenIdentifier {
	/// This special representation is interpreted as the MOA token.
	pub const MOA_REPRESENTATION: &'static [u8] = b"MOA";

	/// New instance of the special MOA token representation.
	pub fn moa() -> Self {
		TokenIdentifier(BoxedBytes::empty())
	}

	#[inline]
	pub fn is_moa(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn is_dct(&self) -> bool {
		!self.is_moa()
	}

	#[inline]
	pub fn into_boxed_bytes(self) -> BoxedBytes {
		self.0
	}

	#[deprecated(
		note = "Please use the as_dct_identifier method instead, its name is more suggestive."
	)]
	#[inline]
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}

	#[inline]
	pub fn as_dct_identifier(&self) -> &[u8] {
		self.0.as_slice()
	}

	#[inline]
	pub fn as_name(&self) -> &[u8] {
		if self.is_moa() {
			TokenIdentifier::MOA_REPRESENTATION
		} else {
			self.0.as_slice()
		}
	}
}

impl AsRef<[u8]> for TokenIdentifier {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

impl From<BoxedBytes> for TokenIdentifier {
	#[inline]
	fn from(boxed_bytes: BoxedBytes) -> Self {
		if boxed_bytes.as_slice() == TokenIdentifier::MOA_REPRESENTATION {
			TokenIdentifier::moa()
		} else {
			TokenIdentifier(boxed_bytes)
		}
	}
}

impl<'a> From<&'a [u8]> for TokenIdentifier {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		if byte_slice == TokenIdentifier::MOA_REPRESENTATION {
			TokenIdentifier::moa()
		} else {
			TokenIdentifier(BoxedBytes::from(byte_slice))
		}
	}
}

impl PartialEq<&[u8]> for TokenIdentifier {
	#[inline]
	fn eq(&self, other: &&[u8]) -> bool {
		if self.is_moa() {
			*other == TokenIdentifier::MOA_REPRESENTATION
		} else {
			self.0.as_slice().eq(*other)
		}
	}
}

impl NestedEncode for TokenIdentifier {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_name().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_name().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for TokenIdentifier {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_name().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_name().top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for TokenIdentifier {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(TokenIdentifier::from(BoxedBytes::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		TokenIdentifier::from(BoxedBytes::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for TokenIdentifier {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(TokenIdentifier::from(BoxedBytes::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		TokenIdentifier::from(BoxedBytes::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for TokenIdentifier {
	fn type_name() -> String {
		"TokenIdentifier".into()
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;
	use dharitri_codec::test_util::*;

	#[test]
	fn test_moa() {
		assert!(TokenIdentifier::moa().is_moa());
	}

	#[test]
	fn test_codec() {
		check_top_encode_decode(
			TokenIdentifier::moa(),
			TokenIdentifier::MOA_REPRESENTATION,
		);
		check_dep_encode_decode(
			TokenIdentifier::moa(),
			dep_encode_to_vec_or_panic(&TokenIdentifier::MOA_REPRESENTATION).as_slice(),
		);

		// also allowed
		assert_eq!(
			TokenIdentifier::moa(),
			check_top_decode::<TokenIdentifier>(&[])
		);
		assert_eq!(
			TokenIdentifier::moa(),
			check_dep_decode::<TokenIdentifier>(&[0, 0, 0, 0])
		);
	}
}
