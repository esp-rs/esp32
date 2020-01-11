#[doc = "Reader of register HOST_SLCHOST_GPIO_STATUS0"]
pub type R = crate::R<u32, super::HOST_SLCHOST_GPIO_STATUS0>;
#[doc = "Writer for register HOST_SLCHOST_GPIO_STATUS0"]
pub type W = crate::W<u32, super::HOST_SLCHOST_GPIO_STATUS0>;
#[doc = "Register HOST_SLCHOST_GPIO_STATUS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_SLCHOST_GPIO_STATUS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOST_GPIO_SDIO_INT0`"]
pub type HOST_GPIO_SDIO_INT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HOST_GPIO_SDIO_INT0`"]
pub struct HOST_GPIO_SDIO_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_GPIO_SDIO_INT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_gpio_sdio_int0(&self) -> HOST_GPIO_SDIO_INT0_R {
        HOST_GPIO_SDIO_INT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_gpio_sdio_int0(&mut self) -> HOST_GPIO_SDIO_INT0_W {
        HOST_GPIO_SDIO_INT0_W { w: self }
    }
}
