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
#[doc = "Reader of field `SYSCON_XTAL_TICK_NUM`"]
pub type SYSCON_XTAL_TICK_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_XTAL_TICK_NUM`"]
pub struct SYSCON_XTAL_TICK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_XTAL_TICK_NUM_W<'a> {
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
    pub fn syscon_xtal_tick_num(&self) -> SYSCON_XTAL_TICK_NUM_R {
        SYSCON_XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_xtal_tick_num(&mut self) -> SYSCON_XTAL_TICK_NUM_W {
        SYSCON_XTAL_TICK_NUM_W { w: self }
    }
}
