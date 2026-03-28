use tonic;
use tonic::Response;
use proto_bin::order_api::order_info_provider_server::OrderInfoProvider;
use proto_bin::order_api::{OrderItemResp, OrderListReq, OrderListResp};

#[derive(Default)]
pub struct OrderInfoProviderImpl{

}

#[tonic::async_trait]
impl OrderInfoProvider for OrderInfoProviderImpl{
    async fn order_list(&self, request: tonic::Request<OrderListReq>) -> Result<tonic::Response<OrderListResp>, tonic::Status> {
        let items = vec![
            OrderItemResp {
                order_id: "ORD_20240301_001".to_string(),
                price: "6999.00".to_string(),
                sku_id: "SKU_IPHONE15_PRO_256_BLACK".to_string(),
                spu_id: "SPU_IPHONE15_PRO".to_string(),
                goods_title: "Apple iPhone 15 Pro 256GB 黑色".to_string(),
            },
            OrderItemResp {
                order_id: "ORD_20240301_001".to_string(),
                price: "1299.00".to_string(),
                sku_id: "SKU_NIKE_AIR_MAX_90_42".to_string(),
                spu_id: "SPU_NIKE_AIR_MAX_90".to_string(),
                goods_title: "Nike Air Max 90 运动鞋 42码".to_string(),
            },
            OrderItemResp {
                order_id: "ORD_20240302_002".to_string(),
                price: "79.00".to_string(),
                sku_id: "SKU_BOOK_RUST_PROGRAMMING".to_string(),
                spu_id: "SPU_BOOK_RUST".to_string(),
                goods_title: "Rust编程权威指南 第2版".to_string(),
            },
            OrderItemResp {
                order_id: "ORD_20240302_002".to_string(),
                price: "2999.00".to_string(),
                sku_id: "SKU_MIJIA_AIR_PURIFIER_PRO".to_string(),
                spu_id: "SPU_MIJIA_AIR_PURIFIER".to_string(),
                goods_title: "小米空气净化器 Pro H".to_string(),
            },
            OrderItemResp {
                order_id: "ORD_20240303_003".to_string(),
                price: "199.00".to_string(),
                sku_id: "SKU_IMPORTED_COFFEE_BEANS_500G".to_string(),
                spu_id: "SPU_COFFEE_BEANS".to_string(),
                goods_title: "进口阿拉比卡咖啡豆 500g 深度烘焙".to_string(),
            },
        ];

        Ok(Response::new(OrderListResp { items }))
    }
}