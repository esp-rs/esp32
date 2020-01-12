#[doc = "Reader of register U4_CONF1"]
pub type R = crate::R<u32, super::U4_CONF1>;
#[doc = "Writer for register U4_CONF1"]
pub type W = crate::W<u32, super::U4_CONF1>;
#[doc = "Register U4_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::U4_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_THRES1_U4`"]
pub type CNT_THRES1_U4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_THRES1_U4`"]
pub struct CNT_THRES1_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES1_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNT_THRES0_U4`"]
pub type CNT_THRES0_U4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_THRES0_U4`"]
pub struct CNT_THRES0_U4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES0_U4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit4."]
    #[inline(always)]
    pub fn cnt_thres1_u4(&self) -> CNT_THRES1_U4_R {
        CNT_THRES1_U4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit4."]
    #[inline(always)]
    pub fn cnt_thres0_u4(&self) -> CNT_THRES0_U4_R {
        CNT_THRES0_U4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit4."]
    #[inline(always)]
    pub fn cnt_thres1_u4(&mut self) -> CNT_THRES1_U4_W {
        CNT_THRES1_U4_W { w: self }
    }
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit4."]
    #[inline(always)]
    pub fn cnt_thres0_u4(&mut self) -> CNT_THRES0_U4_W {
        CNT_THRES0_U4_W { w: self }
    }
}