#[doc = "Reader of register TIMER3"]
pub type R = crate::R<u32, super::TIMER3>;
#[doc = "Writer for register TIMER3"]
pub type W = crate::W<u32, super::TIMER3>;
#[doc = "Register TIMER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROM_RAM_POWERUP_TIMER`"]
pub type ROM_RAM_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROM_RAM_POWERUP_TIMER`"]
pub struct ROM_RAM_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RAM_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `ROM_RAM_WAIT_TIMER`"]
pub type ROM_RAM_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ROM_RAM_WAIT_TIMER`"]
pub struct ROM_RAM_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RAM_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WIFI_POWERUP_TIMER`"]
pub type WIFI_POWERUP_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WIFI_POWERUP_TIMER`"]
pub struct WIFI_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `WIFI_WAIT_TIMER`"]
pub type WIFI_WAIT_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIFI_WAIT_TIMER`"]
pub struct WIFI_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_WAIT_TIMER_W<'a> {
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
    pub fn rom_ram_powerup_timer(&self) -> ROM_RAM_POWERUP_TIMER_R {
        ROM_RAM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rom_ram_wait_timer(&self) -> ROM_RAM_WAIT_TIMER_R {
        ROM_RAM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rom_ram_powerup_timer(&mut self) -> ROM_RAM_POWERUP_TIMER_W {
        ROM_RAM_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rom_ram_wait_timer(&mut self) -> ROM_RAM_WAIT_TIMER_W {
        ROM_RAM_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W {
        WIFI_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W {
        WIFI_WAIT_TIMER_W { w: self }
    }
}
