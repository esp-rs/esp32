#[doc = "Reader of register PRO_VECBASE_SET"]
pub type R = crate::R<u32, super::PRO_VECBASE_SET>;
#[doc = "Writer for register PRO_VECBASE_SET"]
pub type W = crate::W<u32, super::PRO_VECBASE_SET>;
#[doc = "Register PRO_VECBASE_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_VECBASE_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_OUT_VECBASE_REG`"]
pub type PRO_OUT_VECBASE_REG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRO_OUT_VECBASE_REG`"]
pub struct PRO_OUT_VECBASE_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_OUT_VECBASE_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase_reg(&self) -> PRO_OUT_VECBASE_REG_R {
        PRO_OUT_VECBASE_REG_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase_reg(&mut self) -> PRO_OUT_VECBASE_REG_W {
        PRO_OUT_VECBASE_REG_W { w: self }
    }
}