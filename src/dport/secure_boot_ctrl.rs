#[doc = "Reader of register SECURE_BOOT_CTRL"]
pub type R = crate::R<u32, super::SECURE_BOOT_CTRL>;
#[doc = "Writer for register SECURE_BOOT_CTRL"]
pub type W = crate::W<u32, super::SECURE_BOOT_CTRL>;
#[doc = "Register SECURE_BOOT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SECURE_BOOT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SW_BOOTLOADER_SEL`"]
pub type DPORT_SW_BOOTLOADER_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SW_BOOTLOADER_SEL`"]
pub struct DPORT_SW_BOOTLOADER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SW_BOOTLOADER_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_sw_bootloader_sel(&self) -> DPORT_SW_BOOTLOADER_SEL_R {
        DPORT_SW_BOOTLOADER_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_sw_bootloader_sel(&mut self) -> DPORT_SW_BOOTLOADER_SEL_W {
        DPORT_SW_BOOTLOADER_SEL_W { w: self }
    }
}
