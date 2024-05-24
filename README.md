A simple color conversion program that uses actix-web and combines it with dropbox for cloud file storage and mongodb for image information storage.<br>
There are 2 endpoints:<br>
GET: /show_result/<unique_id> - to retrieve the converted image from the server based on the unique image ID<br>
POST: /send_data - with 2 parameters: id (unique image ID) and conversion_type to specify the type of conversion (YCbCr, HSV) - the idea was to first store the image in dropbox, then generate a unique image ID and pass it to this method.<br>
