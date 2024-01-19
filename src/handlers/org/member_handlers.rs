use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::db::connection::establish_connection;
use crate::models::orgs::{
  EditBelong, BelongIdentity, OrgPermission, BelongStaff
};
use crate::configs::state::AppState;
use serde_json::json;
use crate::middlewares::auth::{
  auth_middleware::{JwtMiddleware, Claims},
  role_middleware::check_member_authority
};
use crate::middlewares::org::editing_middleware::*;

// Handler for editing member info
pub async fn edit_member(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, edit_data: web::Json<EditBelong>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    match edit_data.validate() {
      Ok(belong_data) => {

        let req_permission = OrgPermission {
          title: "members".to_owned(),
          name: "delete".to_owned()
        };
        // Check if the user is authorized to perform this action
        match check_member_authority(&user.id, &belong_data.section, &req_permission, &mut conn) {
          Ok(true) => {
            match belong_edited(&belong_data, &mut conn) {
              Ok(belong) => {
                return HttpResponse::Ok().json(
                  json!({
                    "success": true,
                    "belong": belong,
                    "message": "User Details was changed successfully!"
                  })
                )
              }
              Err(Error::NotFound) => {
                return HttpResponse::NotFound().json(
                  json!({
                    "success": false,
                    "message": "Member not found!"
                  })
                )
              }
              Err(_) => {
                return  HttpResponse::InternalServerError().json(
                  json!({
                    "success": false,
                    "message": "Could change the user information: An error occurred during the process!"
                  })
                )
              }
            }
          }

          Ok(false) => {
            return HttpResponse::Unauthorized().json(
              json!({
                "success": false,
                "message": "You're not authorized to perform this action!"
              })
            )
          }
          Err(_) => {
            return  HttpResponse::Unauthorized().json(
              json!({
                "success": false,
                "message": "Could not verify your authority: An error occurred during the process!"
              })
            )
          }
        }
      },
      Err(err) => {
        return HttpResponse::ExpectationFailed().json(
          json!({
            "success": false,
            "message": err.to_string()
          })
        )
      }
    }
	}
	else {
		return HttpResponse::BadRequest().json(
      json!({
        "success": false,
        "message": "Authorization failure!"
      })
    )
	}
}


// Handler for editing member staff status
pub async fn edit_staff_status(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, status_data: web::Json<BelongStaff>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let belong_data = status_data.into_inner();
    let req_permission = OrgPermission {
      title: "staff".to_owned(),
      name: "update".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_member_authority(&user.id, &belong_data.section, &req_permission, &mut conn) {
      Ok(true) => {
        match belong_staff_edited(&belong_data.id, &belong_data.staff, &mut conn) {
          Ok(belong) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "belong": belong,
                "message": "Member Staff status was changed successfully!"
              })
            )
          }
          Err(Error::NotFound) => {
            return HttpResponse::NotFound().json(
              json!({
                "success": false,
                "message": "Member not found!"
              })
            )
          } 
          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not edit member staff status: An error occurred during the process!"
              })
            )
          }
        }
      }

      Ok(false) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(_) => {
        return  HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "Could not verify your authority: An error occurred during the process!"
          })
        )
      }
    }
	}
	else {
		return HttpResponse::BadRequest().json(
      json!({
        "success": false,
        "message": "Authorization failure!"
      })
    )
	}
}


// Handler for deactivating member 
pub async fn remove_member(req: HttpRequest, _: JwtMiddleware, app_data: web::Data<AppState>, status_data: web::Json<BelongIdentity>) -> impl Responder {
  //  Get extensions
  let ext = req.extensions();
  let mut conn = establish_connection(&app_data.config.database_url).await;


  // Use the 'get' method to retrieve the 'Claims' value from extensions
	if let Some(claims) = ext.get::<Claims>() {
		// Access 'user' from 'Claims'
		let user = &claims.user;

    let belong_data = status_data.into_inner();
    let req_permission = OrgPermission {
      title: "staff".to_owned(),
      name: "delete".to_owned()
    };

    // Check if the user is authorized to perform this action
    match check_member_authority(&user.id, &belong_data.section, &req_permission, &mut conn) {
      Ok(true) => {
        match member_removed(&belong_data.author, &belong_data.section, &belong_data.id, &mut conn) {
          Ok(true) => {
            return HttpResponse::Ok().json(
              json!({
                "success": true,
                "message": "User is no longer active member in this org!"
              })
            )
          }
          Ok(false) => {
            return HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not remove member: An error occurred during the process!"
              })
            )
          }
          Err(Error::NotFound) => {
            return HttpResponse::NotFound().json(
              json!({
                "success": false,
                "message": "Member not found!"
              })
            )
          }
          Err(_) => {
            return  HttpResponse::InternalServerError().json(
              json!({
                "success": false,
                "message": "Could not remove member: An error occurred during the process!"
              })
            )
          }
        }
      }

      Ok(false) => {
        return HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "You're not authorized to perform this action!"
          })
        )
      }
      Err(_) => {
        return  HttpResponse::Unauthorized().json(
          json!({
            "success": false,
            "message": "Could not verify your authority: An error occurred during the process!"
          })
        )
      }
    }
	}
	else {
		return HttpResponse::BadRequest().json(
      json!({
        "success": false,
        "message": "Authorization failure!"
      })
    )
	}
}