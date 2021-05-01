MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 56K
  SETTINGS : ORIGIN = 0x0800E800, LENGTH = 2K
  BOOTLOADER: ORIGIN = 0x0800F000, LENGTH = 4K

  RAM_FLAGS : ORIGIN = 0x20000000, LENGTH = 128
  RAM : ORIGIN = 0x20000080, LENGTH = (8K - 128)
}

/* TODO: Can I just have overlapping memory regions?    */
/*                                                      */
/* NOTE: Make sure the size of the app is in sync with  */
/* the `sprocket-boot` repo!                            */

/*
_app_start = ORIGIN(FLASH);
_settings_start = ORIGIN(SETTINGS);
_bootloader_start = ORIGIN(BOOTLOADER);
_ram_flags = ORIGIN(RAM_FLAGS);
*/
