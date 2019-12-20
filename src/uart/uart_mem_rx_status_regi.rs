#[doc = "Reader of register UART_MEM_RX_STATUS_REG(i)"]
pub type R = crate::R<u32, super::UART_MEM_RX_STATUS_REGI>;
#[doc = "Writer for register UART_MEM_RX_STATUS_REG(i)"]
pub type W = crate::W<u32, super::UART_MEM_RX_STATUS_REGI>;
#[doc = "Register UART_MEM_RX_STATUS_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_MEM_RX_STATUS_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_MEM_RX_STATUS`"]
pub type UART_MEM_RX_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_MEM_RX_STATUS`"]
pub struct UART_MEM_RX_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_RX_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `UART_MEM_RX_RD_ADDR`"]
pub type UART_MEM_RX_RD_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_MEM_RX_RD_ADDR`"]
pub struct UART_MEM_RX_RD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_RX_RD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 2)) | (((value as u32) & 0x07ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `UART_MEM_RX_WR_ADDR`"]
pub type UART_MEM_RX_WR_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_MEM_RX_WR_ADDR`"]
pub struct UART_MEM_RX_WR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_RX_WR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 13)) | (((value as u32) & 0x07ff) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register stores the current uart rx mem read address and rx mem write address"]
    #[inline(always)]
    pub fn uart_mem_rx_status(&self) -> UART_MEM_RX_STATUS_R {
        UART_MEM_RX_STATUS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 2:12 - This register stores the rx mem read address"]
    #[inline(always)]
    pub fn uart_mem_rx_rd_addr(&self) -> UART_MEM_RX_RD_ADDR_R {
        UART_MEM_RX_RD_ADDR_R::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:23 - This register stores the rx mem write address"]
    #[inline(always)]
    pub fn uart_mem_rx_wr_addr(&self) -> UART_MEM_RX_WR_ADDR_R {
        UART_MEM_RX_WR_ADDR_R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register stores the current uart rx mem read address and rx mem write address"]
    #[inline(always)]
    pub fn uart_mem_rx_status(&mut self) -> UART_MEM_RX_STATUS_W {
        UART_MEM_RX_STATUS_W { w: self }
    }
    #[doc = "Bits 2:12 - This register stores the rx mem read address"]
    #[inline(always)]
    pub fn uart_mem_rx_rd_addr(&mut self) -> UART_MEM_RX_RD_ADDR_W {
        UART_MEM_RX_RD_ADDR_W { w: self }
    }
    #[doc = "Bits 13:23 - This register stores the rx mem write address"]
    #[inline(always)]
    pub fn uart_mem_rx_wr_addr(&mut self) -> UART_MEM_RX_WR_ADDR_W {
        UART_MEM_RX_WR_ADDR_W { w: self }
    }
}
