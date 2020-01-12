#[doc = "Reader of register TIMER4"]
pub type R = crate::R<u32, super::TIMER4>;
#[doc = "Writer for register TIMER4"]
pub type W = crate::W<u32, super::TIMER4>;
#[doc = "Register TIMER4 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DG_WRAP_POWERUP_TIMER`"]
pub type DG_WRAP_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DG_WRAP_POWERUP_TIMER`"]
pub struct DG_WRAP_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `DG_WRAP_WAIT_TIMER`"]
pub type DG_WRAP_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DG_WRAP_WAIT_TIMER`"]
pub struct DG_WRAP_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `POWERUP_TIMER`"]
pub type POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POWERUP_TIMER`"]
pub struct POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `WAIT_TIMER`"]
pub type WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WAIT_TIMER`"]
pub struct WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn powerup_timer(&self) -> POWERUP_TIMER_R {
        POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wait_timer(&self) -> WAIT_TIMER_R {
        WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W {
        DG_WRAP_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W {
        DG_WRAP_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn powerup_timer(&mut self) -> POWERUP_TIMER_W {
        POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wait_timer(&mut self) -> WAIT_TIMER_W {
        WAIT_TIMER_W { w: self }
    }
}
