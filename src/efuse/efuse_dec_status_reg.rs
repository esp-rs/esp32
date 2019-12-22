#[doc = "Reader of register EFUSE_DEC_STATUS_REG"]
pub type R = crate::R<u32, super::EFUSE_DEC_STATUS_REG>;
#[doc = "Writer for register EFUSE_DEC_STATUS_REG"]
pub type W = crate::W<u32, super::EFUSE_DEC_STATUS_REG>;
#[doc = "Register EFUSE_DEC_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_DEC_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_DEC_WARNINGS`"]
pub type EFUSE_DEC_WARNINGS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_DEC_WARNINGS`"]
pub struct EFUSE_DEC_WARNINGS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DEC_WARNINGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - the decode result of 3/4 coding scheme has warning"]
    #[inline(always)]
    pub fn efuse_dec_warnings(&self) -> EFUSE_DEC_WARNINGS_R {
        EFUSE_DEC_WARNINGS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - the decode result of 3/4 coding scheme has warning"]
    #[inline(always)]
    pub fn efuse_dec_warnings(&mut self) -> EFUSE_DEC_WARNINGS_W {
        EFUSE_DEC_WARNINGS_W { w: self }
    }
}
