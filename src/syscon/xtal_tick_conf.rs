#[doc = "Reader of register XTAL_TICK_CONF"]
pub type R = crate::R<u32, super::XTAL_TICK_CONF>;
#[doc = "Writer for register XTAL_TICK_CONF"]
pub type W = crate::W<u32, super::XTAL_TICK_CONF>;
#[doc = "Register XTAL_TICK_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::XTAL_TICK_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL_TICK_NUM`"]
pub type XTAL_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTAL_TICK_NUM`"]
pub struct XTAL_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_TICK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W {
        XTAL_TICK_NUM_W { w: self }
    }
}
