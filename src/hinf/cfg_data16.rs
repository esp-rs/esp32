#[doc = "Reader of register CFG_DATA16"]
pub type R = crate::R<u32, super::CFG_DATA16>;
#[doc = "Writer for register CFG_DATA16"]
pub type W = crate::W<u32, super::CFG_DATA16>;
#[doc = "Register CFG_DATA16 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_DATA16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVICE_ID_FN2`"]
pub type DEVICE_ID_FN2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEVICE_ID_FN2`"]
pub struct DEVICE_ID_FN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_ID_FN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `USER_ID_FN2`"]
pub type USER_ID_FN2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `USER_ID_FN2`"]
pub struct USER_ID_FN2_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_ID_FN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&self) -> DEVICE_ID_FN2_R {
        DEVICE_ID_FN2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&self) -> USER_ID_FN2_R {
        USER_ID_FN2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&mut self) -> DEVICE_ID_FN2_W {
        DEVICE_ID_FN2_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&mut self) -> USER_ID_FN2_W {
        USER_ID_FN2_W { w: self }
    }
}
